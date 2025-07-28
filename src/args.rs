use clap::Parser;

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "charfreq-rs")]
#[command(about = "A project by github/jamesukiyo\n\nAnalyse character frequencies in a repository.", long_about = None)]
pub struct Args {
	/// Path to the repository
	#[arg(short = 'd', long = "dir")]
	pub repo_path: String,

	/// Number of top characters to display
	#[arg(short = 't', long = "top", default_value = "20")]
	pub top: usize,

	/// Include spaces and whitespace characters in the output
	#[arg(short = 's', long = "show-spaces")]
	pub show_spaces: bool,

	/// Exclude all letters (A-Z, a-z) from the output
	#[arg(short = 'e', long = "exclude-letters")]
	pub exclude_letters: bool,

	/// Save results as CSV in the current working directory
	#[arg(short = 'c', long = "csv")]
	pub save_csv: bool,

	/// Show files with errors during the scan (usually invalid file types)
	#[arg(short = 'v', long = "verbose")]
	pub verbose: bool,

	/// Additional filetypes to ignore (comma-separated or once for each
	/// filetype)
	#[arg(short = 'i', long = "ignore", value_delimiter = ',')]
	pub ignore_filetypes: Vec<String>,

	/// Additional directories to ignore (comma-separated or once for each
	/// directory)
	#[arg(short = 'I', long = "ignore-dir", value_delimiter = ',')]
	pub ignore_dirs: Vec<String>,
}
