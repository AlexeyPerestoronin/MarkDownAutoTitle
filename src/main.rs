use clap::Parser;
use std::{
    error::Error,
    path::{PathBuf},
};

mod generator;

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0.0",
    about = "Command Line Tool for auto-adding title to mark-down file"
)]
struct Args {
    /// Target markdown file for title addition
    #[arg(long)]
    file: PathBuf,

    /// The message which should be used as the title
    #[arg(long, default_value = "Auto-Title:")]
    title_message: String,

    /// Title message for title addition
    #[arg(long, default_value_t = 4)]
    tab_space_size: u8,

    /// Result markdown file (with generated title) (if not defined, equals the target)
    #[arg(long)]
    result_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Args::parse();

    generator::MarkDownTitleGenerator::new(
        &arguments.file,
        arguments.title_message,
        arguments.tab_space_size,
    )?
    .generate()?
    .finish(&arguments.result_file.unwrap_or(arguments.file))?;

    Ok(())
}
