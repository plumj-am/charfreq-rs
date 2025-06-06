use std::{
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct CharFrequency {
	pub character: char,
	pub count: u64,
}

#[derive(Debug)]
pub struct ScanResult {
	pub char_frequencies: Vec<CharFrequency>,
	pub total_chars: u64,
	pub files_processed: u64,
	pub error_files: Vec<String>,
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

// Helper enum to return consistent results from parallel tasks
enum ScanTaskResult {
	FileScanned {
		char_count: HashMap<char, u64>,
		files_processed: u64,
	},
	DirectoryScanned {
		char_count: HashMap<char, u64>,
		files_processed: u64,
		error_files: Vec<String>,
	},
	FileError(String),
}

pub fn scan_repo(
	repo_path: &str,
) -> Result<ScanResult, Box<dyn std::error::Error>> {
	let path = PathBuf::from(repo_path);

	if !path.exists() {
		return Err(format!("Path '{repo_path}' does not exist").into());
	}

	let (char_count, files_processed, error_files) = scan_directory(&path)?;

	let total_chars: u64 = char_count.values().sum();

	let mut char_frequencies: Vec<CharFrequency> = char_count
		.into_iter()
		.map(|(character, count)| CharFrequency { character, count })
		.collect();

	char_frequencies.sort_by(|a, b| b.count.cmp(&a.count));

	Ok(ScanResult {
		char_frequencies,
		total_chars,
		files_processed,
		error_files,
	})
}

// TODO: reduce type complexity (clippy)
fn scan_directory(
	dir_path: &Path,
) -> Result<(HashMap<char, u64>, u64, Vec<String>), Box<dyn std::error::Error>>
{
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
						let mut local_char_count = HashMap::new();
						for ch in content.chars() {
							*local_char_count.entry(ch).or_insert(0) += 1;
						}
						Some(ScanTaskResult::FileScanned {
							char_count: local_char_count,
							files_processed: 1,
						})
					}
					Err(e) => Some(ScanTaskResult::FileError(format!(
						"{}: {}",
						path.display(),
						e
					))),
				}
			} else if path.is_dir() {
				scan_directory(&path).ok().map(
					|(char_count, files_processed, error_files)| {
						ScanTaskResult::DirectoryScanned {
							char_count,
							files_processed,
							error_files,
						}
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
			ScanTaskResult::FileScanned {
				char_count,
				files_processed,
			} => {
				for (ch, count) in char_count {
					*final_char_count.entry(ch).or_insert(0) += count;
				}
				final_files_processed += files_processed;
			}
			ScanTaskResult::DirectoryScanned {
				char_count,
				files_processed,
				error_files,
			} => {
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

	Ok((final_char_count, final_files_processed, final_error_files))
}
