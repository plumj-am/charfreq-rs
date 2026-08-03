use std::{
	collections::HashMap,
	fs::{self, File},
	io::{self, BufReader, Read},
	path::{Path, PathBuf},
};

use rayon::prelude::*;

use super::args::Args;

pub type ScanError = io::Error;

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
struct CharCounts {
	ascii: [u64; 128],
	unicode: HashMap<char, u64>,
}

impl CharCounts {
	fn new() -> Self {
		CharCounts {
			ascii: [0; 128],
			unicode: HashMap::new(),
		}
	}

	fn merge(&mut self, other: CharCounts) {
		for i in 0..128 {
			self.ascii[i] += other.ascii[i];
		}
		for (ch, count) in other.unicode {
			*self.unicode.entry(ch).or_insert(0) += count;
		}
	}
}

#[derive(Debug)]
struct DirScanData {
	char_count: CharCounts,
	files_processed: u64,
	error_files: Vec<String>,
}

#[derive(Debug)]
struct FileScanData {
	char_count: CharCounts,
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
		return Err(io::Error::new(
			io::ErrorKind::NotFound,
			format!("Path '{repo_path}' does not exist"),
		));
	}

	let DirScanData {
		char_count,
		files_processed,
		error_files,
	} = scan_directory(&path, args)?;

	let total_chars: u64 = char_count.ascii.iter().sum::<u64>()
		+ char_count.unicode.values().sum::<u64>();

	let mut char_frequencies: Vec<CharFreq> = Vec::with_capacity(128);
	for (i, &count) in char_count.ascii.iter().enumerate() {
		if count > 0 {
			char_frequencies.push(CharFreq {
				character: i as u8 as char,
				count,
			});
		}
	}
	for (character, count) in char_count.unicode {
		char_frequencies.push(CharFreq { character, count });
	}

	char_frequencies.sort_by_key(|a| std::cmp::Reverse(a.count));

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
						let mut content = Vec::with_capacity(32 * 1024);
						match reader.read_to_end(&mut content) {
							Ok(_) => match count_chars(&content) {
								Ok(local_char_count) => Some(
									ScanTaskResult::FileScanned(FileScanData {
										char_count: local_char_count,
										files_processed: 1,
									}),
								),
								Err(e) => Some(ScanTaskResult::FileError(
									format!("{}: {}", path.display(), e),
								)),
							},
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

	let mut final_char_count = CharCounts::new();
	let mut final_files_processed = 0u64;
	let mut final_error_files: Vec<String> = Vec::new();

	for task_result in all_task_results {
		match task_result {
			ScanTaskResult::FileScanned(FileScanData {
				char_count,
				files_processed,
			}) => {
				final_char_count.merge(char_count);
				final_files_processed += files_processed;
			}
			ScanTaskResult::DirScanned(DirScanData {
				char_count,
				files_processed,
				error_files,
			}) => {
				final_char_count.merge(char_count);
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

fn count_chars(content: &[u8]) -> Result<CharCounts, std::str::Utf8Error> {
	// Validate UTF-8, then count characters, splitting ASCII chars into the
	// dense array and the rest into the map. The dense array keeps ASCII
	// counting alloc-free; only non-ASCII content touches the HashMap.
	let text = std::str::from_utf8(content)?;
	let mut counts = CharCounts::new();
	for ch in text.chars() {
		let code = ch as u32;
		if code < 128 {
			counts.ascii[code as usize] += 1;
		} else {
			*counts.unicode.entry(ch).or_insert(0) += 1;
		}
	}
	Ok(counts)
}
