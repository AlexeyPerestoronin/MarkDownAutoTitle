use clap::Parser;
use std::{fs::OpenOptions, path::Path};

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0.0",
    about = "Command Line Tool for auto-adding title to mark-down file"
)]
struct Args {
    /// Target mark-down file for title addition
    #[arg(short, long)]
    file_name: String,

    /// Title message for title addition
    #[arg(long, default_value = "Auto-Title:")]
    title_message: String,

    /// Whether to add links to navigation between titles
    #[arg(long, default_value_t = false)]
    with_links: bool,
}

fn main() {
    let arguments = Args::parse();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(Path::new(&arguments.file_name))
        .unwrap();

    // println!("CLI count is {}", arguments.count);
    println!("CLI file-name is {}", arguments.file_name);
}
