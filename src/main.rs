use clap::Parser;
use regex::Regex;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

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

    /// Whether to print information about process in console
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let arguments = Args::parse();

    let titles_info = detect_titles(Path::new(&arguments.file_name));
    for title_info in &titles_info {
        println!("line number: {}", title_info.line_number);
        println!("title: {}", title_info.title);
        println!("level: {}", title_info.level);
    }

    // println!("CLI count is {}", arguments.count);
    println!("CLI file-name is {}", arguments.file_name);
}

struct TitleInfo {
    line_number: usize,
    title: String,
    level: usize,
}

fn detect_titles(file_path: &Path) -> Vec<TitleInfo> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(file_path)
        .unwrap();

    let reader = BufReader::new(file);
    let reg_title_pattern = Regex::new(r"(?<level>#+)\s+(?<title>.+)").unwrap();
    let mut titles_info: Vec<TitleInfo> = vec![];
    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if let Some(capture) = reg_title_pattern.captures(line.as_str()) {
                titles_info.push(TitleInfo {
                    line_number: line_number,
                    title: String::from(&capture["title"]),
                    level: *&capture["level"].chars().filter(|ch| *ch == '#').count(),
                });
            }
        }
    }

    return titles_info;
}
