use std::{
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

use rayon::prelude::*;

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

// TODO: Add option to include additional filetypes
fn should_skip_file(filepath: &Path) -> bool {
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

	// TODO: Add option to include additional directories
	#[rustfmt::skip]
	let skip_dirs = [
		"node_modules", "venv", "env", ".git",
		".svelte-kit", ".mvn", "__pycache__", "build",
		"dist", ".idea", ".husky", ".turbo",
		"target", ".vscode",
	];

	// Check the file extension
	if let Some(ext) = filepath.extension()
		&& let Some(ext_str) = ext.to_str()
		&& skip_extensions
			.iter()
			.any(|&e| ext_str.eq_ignore_ascii_case(&e[1..]))
	{
		return true;
	}

	// Check if any part of the path contains directories to skip
	for component in filepath.components() {
		if let Some(dir_name) = component.as_os_str().to_str()
			&& skip_dirs.contains(&dir_name)
		{
			return true;
		}
	}

	false
}

pub fn scan_repo(repo_path: &str) -> Result<FinalOutput, ScanError> {
	let path = PathBuf::from(repo_path);

	if !path.exists() {
		return Err(format!("Path '{repo_path}' does not exist").into());
	}

	let DirScanData {
		char_count,
		files_processed,
		error_files,
	} = scan_directory(&path)?;

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

fn scan_directory(dir_path: &Path) -> Result<DirScanData, ScanError> {
	let entries: Vec<_> =
		fs::read_dir(dir_path)?.collect::<Result<Vec<_>, _>>()?;

	let all_task_results: Vec<ScanTaskResult> = entries
		.into_par_iter()
		.filter_map(|entry| {
			let path = entry.path();

			if should_skip_file(&path) {
				return None;
			}

			if path.is_file() {
				match fs::read_to_string(&path) {
					Ok(content) => {
						let local_char_count = count_chars(&content);
						Some(ScanTaskResult::FileScanned(FileScanData {
							char_count: local_char_count,
							files_processed: 1,
						}))
					}
					Err(e) => Some(ScanTaskResult::FileError(format!(
						"{}: {}",
						path.display(),
						e
					))),
				}
			} else if path.is_dir() {
				scan_directory(&path).ok().map(
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
	let mut char_count = HashMap::new();

	// Faster handling for ascii content
	if content.is_ascii() {
		let mut ascii_counts = [0u64; 128];

		// Can now process bytes directly if ascii
		for &byte in content.as_bytes() {
			ascii_counts[byte as usize] += 1;
		}

		// Only insert the non-zero counts to HashMap
		for (i, &count) in ascii_counts.iter().enumerate() {
			if count > 0 {
				char_count.insert(i as u8 as char, count);
			}
		}
	} else {
		// Fallback for non-ascii (unicode) content - this is how it was done
		// for all characters previously.
		for ch in content.chars() {
			*char_count.entry(ch).or_insert(0) += 1;
		}
	}

	char_count
}
