use std::{fs, io::Write, path::PathBuf, time::Duration};

use crate::args::Args;
use crate::scanner::ScanResult;

pub fn print_results(
	result: &ScanResult,
	args: &Args,
	scan_time: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("\nProcessed {} files", result.files_processed);
	println!("Total characters: {:?}", result.total_chars);
	println!("Scan time: {:.2}s", scan_time.as_secs_f64());

	println!("\nTop character frequencies:");
	println!("Char | Count | Percentage");
	println!("{}", "-".repeat(30));

	let mut csv_results = Vec::new();
	let mut displayed_count = 0;

	for freq in &result.char_frequencies {
		// Apply filters from the CLI options
		if args.exclude_letters && freq.character.is_alphabetic() {
			continue;
		}

		if !args.show_spaces && freq.character.is_whitespace() {
			continue;
		}

		// Format certain characters for so they'll display correctly
		let char_display = if freq.character.is_whitespace() {
			match freq.character {
				' ' => "' '".to_string(),
				'\n' => "'\\n'".to_string(),
				'\t' => "'\\t'".to_string(),
				'\r' => "'\\r'".to_string(),
				_ => format!("'{}'", freq.character.escape_debug()),
			}
		} else {
			freq.character.to_string()
		};

		// Occurrence of characters as a percentage
		let percentage =
			(freq.count as f64 / result.total_chars as f64) * 100.0;

		println!(
			"{:>4} | {:>7?} | {:>6.2}%",
			char_display, freq.count, percentage
		);

		// WARN: Has *not* been tested well
		if args.save_csv {
			csv_results.push((char_display, freq.count, percentage));
		}

		displayed_count += 1;
		if displayed_count >= args.top {
			break;
		}
	}

	// WARN: Has *not* been tested well
	if args.save_csv {
		save_csv(&csv_results)?;
	}

	if !result.error_files.is_empty() {
		println!("\nFiles with errors:");
		for error in &result.error_files {
			println!("{error}");
		}
	}

	Ok(())
}

// WARN: Has *not* been tested well
fn save_csv(
	results: &[(String, u64, f64)],
) -> Result<(), Box<dyn std::error::Error>> {
	let csv_path = PathBuf::from("char_freqs.csv");
	let mut file = fs::File::create(&csv_path)?;

	writeln!(file, "Character,Count,Percentage")?;

	for (character, count, percentage) in results {
		writeln!(file, "{character},{count},{percentage:.2}")?;
	}

	println!("\nResults saved to {}", csv_path.display());
	Ok(())
}
