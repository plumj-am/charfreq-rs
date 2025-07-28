use std::{
	collections::HashMap,
	fs::{self, File},
	io::{BufReader, Read},
	path::{Path, PathBuf},
};

use rayon::prelude::*;

use super::args::Args;

pub type ScanError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct CharFreq {
	pub character: char,
	pub count: u64,
}

#[derive(Debug)]
pub struct FinalOutput {
	pub char_frequencies: Vec<CharFreq>,
	pub total_chars: u64,
	pub files_processed: u64,
	pub error_files: Vec<String>,
}

#[derive(Debug)]
struct DirScanData {
	char_count: HashMap<char, u64>,
	files_processed: u64,
	error_files: Vec<String>,
}

#[derive(Debug)]
struct FileScanData {
	char_count: HashMap<char, u64>,
	files_processed: u64,
}

enum ScanTaskResult {
	FileScanned(FileScanData),
	DirScanned(DirScanData),
	FileError(String),
}

fn should_skip_file(filepath: &Path, args: &Args) -> bool {
	#[rustfmt::skip]
	let skip_extensions = [
		".pyc", ".exe", ".dll", ".so",
		".dylib", ".json", ".jpg", ".jpeg",
		".png", ".gif", ".ico", ".svg",
		".pdf", ".zip", ".tar", ".gz",
		".7z", ".webp", ".mp3", ".mp4",
		".avi", ".mov", ".yaml", ".jar",
		".ttf", ".woff", ".woff2", ".ipynb",
		".pkl", ".h5", ".model", ".txt",
		".class", ".tree", ".map", ".debug",
	];

	#[rustfmt::skip]
	let skip_dirs = [
		"node_modules", "venv", "env", ".git",
		".svelte-kit", ".mvn", "__pycache__", "build",
		"dist", ".idea", ".husky", ".turbo",
		"target", ".vscode",
	];

	// check the file extension
	if let Some(ext) = filepath.extension()
		&& let Some(ext_str) = ext.to_str()
	{
		// check default extensions
		if skip_extensions
			.iter()
			.any(|e| ext_str.eq_ignore_ascii_case(&e[1..]))
		{
			return true;
		}

		// check user extensions
		if args.ignore_filetypes.iter().any(|pattern| {
			// handle patterns with/without leading dot
			let pattern = pattern.trim_start_matches('.');
			ext_str.eq_ignore_ascii_case(pattern)
		}) {
			return true;
		}
	}

	// check if any part of the path contains directories to skip
	for component in filepath.components() {
		if let Some(dir_name) = component.as_os_str().to_str() {
			// check default directories
			if skip_dirs.contains(&dir_name) {
				return true;
			}

			// check user-provided directories
			if args.ignore_dirs.iter().any(|d| d == dir_name) {
				return true;
			}
		}
	}

	false
}
pub fn scan_repo(
	repo_path: &str,
	args: &Args,
) -> Result<FinalOutput, ScanError> {
	let path = PathBuf::from(repo_path);

	if !path.exists() {
		return Err(format!("Path '{repo_path}' does not exist").into());
	}

	let DirScanData {
		char_count,
		files_processed,
		error_files,
	} = scan_directory(&path, args)?;

	let total_chars: u64 = char_count.values().sum();

	let mut char_frequencies: Vec<CharFreq> = char_count
		.into_iter()
		.map(|(character, count)| CharFreq { character, count })
		.collect();

	char_frequencies.sort_by(|a, b| b.count.cmp(&a.count));

	Ok(FinalOutput {
		char_frequencies,
		total_chars,
		files_processed,
		error_files,
	})
}

fn scan_directory(
	dir_path: &Path,
	args: &Args,
) -> Result<DirScanData, ScanError> {
	let entries: Vec<_> =
		fs::read_dir(dir_path)?.collect::<Result<Vec<_>, _>>()?;

	let all_task_results: Vec<ScanTaskResult> = entries
		.into_par_iter()
		.filter_map(|entry| {
			let path = entry.path();

			if should_skip_file(&path, args) {
				return None;
			}

			if path.is_file() {
				match File::open(&path) {
					Ok(file) => {
						let mut reader =
							BufReader::with_capacity(32 * 1024, file);
						let mut content = String::with_capacity(32 * 1024);
						match reader.read_to_string(&mut content) {
							Ok(_) => {
								let local_char_count = count_chars(&content);
								Some(ScanTaskResult::FileScanned(
									FileScanData {
										char_count: local_char_count,
										files_processed: 1,
									},
								))
							}
							Err(e) => Some(ScanTaskResult::FileError(format!(
								"{}: {}",
								path.display(),
								e
							))),
						}
					}
					Err(e) => Some(ScanTaskResult::FileError(format!(
						"{}: {}",
						path.display(),
						e
					))),
				}
			} else if path.is_dir() {
				scan_directory(&path, args).ok().map(
					|DirScanData {
					     char_count,
					     files_processed,
					     error_files,
					 }| {
						ScanTaskResult::DirScanned(DirScanData {
							char_count,
							files_processed,
							error_files,
						})
					},
				)
			} else {
				None
			}
		})
		.collect();

	let mut final_char_count: HashMap<char, u64> = HashMap::new();
	let mut final_files_processed = 0u64;
	let mut final_error_files: Vec<String> = Vec::new();

	for task_result in all_task_results {
		match task_result {
			ScanTaskResult::FileScanned(FileScanData {
				char_count,
				files_processed,
			}) => {
				for (ch, count) in char_count {
					*final_char_count.entry(ch).or_insert(0) += count;
				}
				final_files_processed += files_processed;
			}
			ScanTaskResult::DirScanned(DirScanData {
				char_count,
				files_processed,
				error_files,
			}) => {
				for (ch, count) in char_count {
					*final_char_count.entry(ch).or_insert(0) += count;
				}
				final_files_processed += files_processed;
				final_error_files.extend(error_files);
			}
			ScanTaskResult::FileError(error) => {
				final_error_files.push(error);
			}
		}
	}

	Ok(DirScanData {
		char_count: final_char_count,
		files_processed: final_files_processed,
		error_files: final_error_files,
	})
}

fn count_chars(content: &str) -> HashMap<char, u64> {
	let mut char_count = HashMap::with_capacity(128);

	// Faster handling for ascii content
	if content.is_ascii() {
		let mut ascii_counts = [0u64; 128];

		// Can now process bytes directly if ascii
		let bytes = content.as_bytes();
		let chunks = bytes.chunks_exact(8);
		let remainder = chunks.remainder();

		// Process 8 bytes at a time
		for chunk in chunks {
			for &byte in chunk {
				ascii_counts[byte as usize] += 1;
			}
		}

		// Process remaining bytes
		for &byte in remainder {
			ascii_counts[byte as usize] += 1;
		}

		// Only insert the non-zero counts to HashMap
		for (i, &count) in ascii_counts.iter().enumerate() {
			if count > 0 {
				char_count.insert(i as u8 as char, count);
			}
		}
	} else {
		// Pre-allocate space for unicode chars
		for ch in content.chars() {
			*char_count.entry(ch).or_insert(0) += 1;
		}
	}

	char_count
}
