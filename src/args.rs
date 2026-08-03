#[derive(pound::Parse)]
#[pound(name = "charfreq")]
#[pound(about = "Analyse character frequencies in a repository", long_about = None)]
pub struct Args {
   /// Path to the repository
   pub repo_path: String,

   /// Number of top characters to display [default = 20]
   #[pound(short, long, default = "20")]
   pub top: usize,

   /// Include spaces and whitespace characters in the output
   #[pound(short, long)]
   pub show_spaces: bool,

   /// Exclude all letters from the output
   #[pound(short, long)]
   pub exclude_letters: bool,

   /// Save results as a CSV in the current directory
   #[pound(short, long)]
   pub save_csv: bool,

   /// Show files with errors during the scan
   #[pound(short, long)]
   pub verbose: bool,

   /// Additional filetypes to ignore (repeatable)
   #[pound(short, long)]
   pub ignore_filetypes: Vec<String>,

   /// Additional directories to ignore (repeatable)
   #[pound(short = 'I', long, value_delimiter = ',')]
   pub ignore_dirs: Vec<String>,
}
