use once_cell::sync::Lazy;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    collections::LinkedList,
    error::Error,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    option::Option,
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

    pub fn generate(&self, skip_first_title: bool) -> Result<&Self, Box<dyn Error>> {
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
        let mut inside_first_title = false;
        let mut first_title_skipped = !skip_first_title;

        for line in BufReader::new(source).lines() {
            let line = line?;
            if let Some(capture) = TITLE_REGEX_DETECT_PATTERN.captures(&line) {
                if !first_title_skipped {
                    // Skip the first title and its content
                    inside_first_title = true;
                    first_title_skipped = true;
                    continue;
                } else {
                    // If we encounter another title, we are no longer inside the first title's section
                    inside_first_title = false;
                }
                let title = &capture["title"];
                let tab_level = capture["level"].chars().count() - 1;
                let tab_indent = " ".repeat(self.tab_space_size.into());
                let indent = tab_indent.repeat(tab_level);
                titles.push_back(format!("{}* {}", indent, title));
            }
            if !inside_first_title {
                body.push_back(line);
            }
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

impl Drop for MarkDownTitleGenerator {
    fn drop(&mut self) {
        if self.temporary_file_path.exists() {
            fs::remove_file(&self.temporary_file_path).unwrap();
        }
    }
}

/// Adds a SHA-256 hash of the file name to the file name itself, preserving the extension if present.
///
/// # Arguments
///
/// * `path` - A reference to a `Path` representing the original file path.
///
/// # Returns
///
/// * `Option<PathBuf>` - A new `PathBuf` with the modified file name containing the hash, or `None` if the file name or extension cannot be converted to a string.
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

#[cfg(test)]
mod tests {
    use super::*;
    use diff;
    use std::fs;
    use std::path::Path;

    fn compare_content_of_two_files(file_1: &Path, file_2: &Path) -> Result<bool, Box<dyn Error>> {
        let expected_content = fs::read_to_string(file_1)?;
        let expected_content = expected_content.replace("\r\n", "\n");
        let expected_content = expected_content.as_str();

        let real_content = fs::read_to_string(file_2)?;
        let real_content = real_content.replace("\r\n", "\n");
        let real_content = real_content.as_str();

        let mut is_diff_detected = false;
        for diff in diff::chars(expected_content, real_content) {
            match diff {
                // Character in string1 but not in string2
                diff::Result::Left(l) => {
                    println!("-{:?}", l);
                    is_diff_detected = true;
                }
                // Character in string2 but not in string1
                diff::Result::Right(r) => {
                    println!("+{:?}", r);
                    is_diff_detected = true;
                }
                // Ignore characters present in both strings
                diff::Result::Both(_, _) => (),
            }
        }

        Ok(is_diff_detected)
    }

    #[test]
    fn test_0_simple() -> Result<(), Box<dyn Error>> {
        let target_file = Path::new("test_files/test-0_simple (target).md");
        let expected_result_file = Path::new("test_files/test-0_simple (result).md");
        let real_result_file = Path::new("test_files/temp_result.md");

        MarkDownTitleGenerator::new(target_file, "Auto-Title:".to_string(), 4)?
            .generate(false)?
            .finish(&real_result_file)?;

        assert!(
            !compare_content_of_two_files(&expected_result_file, &real_result_file)?,
            "`{:?}` content is not equal of `{:?}` content",
            expected_result_file,
            real_result_file
        );

        // Clean up the temporary file
        fs::remove_file(real_result_file)?;
        Ok(())
    }

    #[test]
    fn test_1_complex() -> Result<(), Box<dyn Error>> {
        let target_file = Path::new("test_files/test-1_complex (target).md");
        let expected_result_file = Path::new("test_files/test-1_complex (result).md");
        let real_result_file = Path::new("test_files/temp_result.md");

        MarkDownTitleGenerator::new(target_file, "Auto-Title:".to_string(), 4)?
            .generate(false)?
            .finish(&real_result_file)?;

        assert!(
            !compare_content_of_two_files(&expected_result_file, &real_result_file)?,
            "`{:?}` content is not equal of `{:?}` content",
            expected_result_file,
            real_result_file
        );

        // Clean up the temporary file
        fs::remove_file(real_result_file)?;
        Ok(())
    }

    #[test]
    fn test_2_with_title() -> Result<(), Box<dyn Error>> {
        let target_file = Path::new("test_files/test-2_with_title (target).md");
        let expected_result_file = Path::new("test_files/test-2_with_title (result).md");
        let real_result_file = Path::new("test_files/temp_result.md");

        MarkDownTitleGenerator::new(target_file, "Auto-Title:".to_string(), 4)?
            .generate(true)?
            .finish(&real_result_file)?;

        assert!(
            !compare_content_of_two_files(&expected_result_file, &real_result_file)?,
            "`{:?}` content is not equal of `{:?}` content",
            expected_result_file,
            real_result_file
        );

        // Clean up the temporary file
        fs::remove_file(real_result_file)?;
        Ok(())
    }
}
