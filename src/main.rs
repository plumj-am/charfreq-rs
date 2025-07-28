use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod args;
mod scanner;
mod utils;

use std::time::Instant;

use args::Args;
use clap::Parser;

fn main() -> Result<(), scanner::ScanError> {
	let args = Args::parse();

	println!("Scanning repository: {}", args.repo_path);

	let start_time = Instant::now();
	let result = scanner::scan_repo(&args.repo_path, &args)?;
	let scan_time = start_time.elapsed();

	utils::print_results(&result, &args, scan_time)?;

	Ok(())
}
