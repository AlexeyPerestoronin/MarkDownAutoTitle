use clap::Parser;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    collections::LinkedList,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0.0",
    about = "Command Line Tool for auto-adding title to mark-down file"
)]
struct Args {
    /// Target markdown file for title addition
    #[arg(short, long)]
    file_name: String,

    /// The message which should be used as the title
    #[arg(long, default_value = "Auto-Title:")]
    title_message: String,

    /// Title message for title addition
    #[arg(long, default_value_t = 4)]
    tab_space_size: u8,

    // TODO: need to implement
    // /// Whether to add links to navigation between titles
    // #[arg(long, default_value_t = false)]
    // with_links: bool,

    // TODO: need to implement
    // /// Whether to detect existing title
    // #[arg(long, default_value_t = false)]
    // detect_existing_title: bool,
    
    // TODO: need to implement
    // /// Whether to print information about process in console
    // #[arg(long, default_value_t = false)]
    // verbose: bool,
}

fn main() {
    let arguments = Args::parse();

    detect_titles(
        Path::new(&arguments.file_name),
        &arguments.title_message,
        arguments.tab_space_size,
    );
}

fn detect_titles(file_path: &Path, title_message: &String, tab_space_size: u8) {
    let source = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file_path)
        .unwrap();

    let mut target = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(add_hash_to_filename(file_path).unwrap())
        .unwrap();

    let reader = BufReader::new(source);
    let reg_title_pattern = Regex::new(r"(?<level>#+)\s+(?<title>.+)").unwrap();
    let mut titles: LinkedList<String> = LinkedList::new();
    titles.push_back(format!("# {title_message}"));
    let mut body: LinkedList<String> = LinkedList::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(capture) = reg_title_pattern.captures(line.as_str()) {
            let title = &capture["title"];
            let tab_level = *&capture["level"].chars().filter(|ch| *ch == '#').count() - 1;
            let tab_indent = " ".repeat(tab_space_size.into());
            let indent = tab_indent.repeat(tab_level);
            titles.push_back(format!("{indent}* {title}"));
        }
        body.push_back(line);
    }

    for title in titles {
        target.write(title.as_bytes()).unwrap();
        target.write(b"\n").unwrap();
    }

    target.write(b"\n").unwrap();

    for title in body {
        target.write(title.as_bytes()).unwrap();
        target.write(b"\n").unwrap();
    }
}

fn add_hash_to_filename(path: &Path) -> Option<PathBuf> {
    // Step 1: Extract file name and extension
    let file_stem = path.file_stem()?.to_str()?;
    let extension = path.extension().and_then(|ext| ext.to_str());

    // Step 2: Generate a hash from the file name
    let hash = {
        let mut hasher = Sha256::new();
        hasher.update(file_stem);
        format!("{:x}", hasher.finalize())
    };

    // Step 3: Construct the new file name
    let new_file_name = match extension {
        Some(ext) => format!("{}_{}.{}", file_stem, hash, ext),
        None => format!("{}_{}", file_stem, hash),
    };

    // Step 4: Update the PathBuf with the new file name
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(new_file_name);

    Some(new_path)
}

// TitleFormer
//  format = detect → change → insert
