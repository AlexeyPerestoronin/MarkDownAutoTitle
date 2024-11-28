use once_cell::sync::Lazy;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    option::Option,
    collections::LinkedList,
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub struct MarkDownTitleGenerator {
    file_path: PathBuf,
    temporary_file_path: PathBuf,
    title_message: String,
    tab_space_size: u8,
}

impl MarkDownTitleGenerator {
    pub fn new(
        file_path: &Path,
        title_message: String,
        tab_space_size: u8,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(MarkDownTitleGenerator {
            file_path: file_path.into(),
            temporary_file_path: add_hash_to_filename(&file_path).unwrap(),
            title_message: title_message,
            tab_space_size: tab_space_size,
        })
    }

    pub fn generate(&self) -> Result<&Self, Box<dyn Error>> {
        static TITLE_REGEX_DETECT_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<level>#+)\s+(?<title>.+)").unwrap());

        // Step-1: open target markdown file
        let source = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .truncate(false)
            .open(&self.file_path)?;

        // Step-2: open (create) temporary markdown file
        let mut temporary = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.temporary_file_path)?;

        // Step-3: preparing new content for the file
        let mut titles: LinkedList<String> = LinkedList::new();
        titles.push_back(format!("# {}", self.title_message));
        let mut body: LinkedList<String> = LinkedList::new();

        for line in BufReader::new(source).lines() {
            let line = line?;
            if let Some(capture) = TITLE_REGEX_DETECT_PATTERN.captures(&line) {
                let title = &capture["title"];
                let tab_level = capture["level"].chars().count() - 1;
                let tab_indent = " ".repeat(self.tab_space_size.into());
                let indent = tab_indent.repeat(tab_level);
                titles.push_back(format!("{}* {}", indent, title));
            }
            body.push_back(line);
        }

        // Step-3: save content to temporary file
        for title in titles {
            writeln!(temporary, "{}", title)?;
        }

        writeln!(temporary)?;

        for line in body {
            writeln!(temporary, "{}", line)?;
        }

        Ok(self)
    }

    pub fn finish(&self, finish_file: &Path) -> Result<(), Box<dyn Error>> {
        {
            // Step-1: open target markdown file
            let mut source = OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .truncate(true)
                .open(finish_file)?;

            // Step-2: open (create) temporary markdown file
            let temporary = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .truncate(false)
                .open(&self.temporary_file_path)?;

            // Step-3: replace content of source file by content of target (with title)
            for line in BufReader::new(temporary).lines() {
                writeln!(source, "{}", line?)?;
            }
        }

        // Step-4: removing temporary file
        std::fs::remove_file(&self.temporary_file_path)?;
        Ok(())
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
