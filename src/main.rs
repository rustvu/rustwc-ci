//! Word count program in Rust
#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

use clap::Parser;

/// Command line arguments
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Files to count (default: stdin)
    #[arg(name = "FILE")]
    filenames: Vec<PathBuf>,

    /// Count lines
    #[arg(short, long)]
    lines: bool,

    /// Count words
    #[arg(short, long)]
    words: bool,

    /// Count characters
    #[arg(short, long)]
    chars: bool,
}

/// File information
#[derive(Debug, Default, Clone, Copy)]
struct FileInfo {
    /// Number of lines
    lines: usize,
    /// Number of words
    words: usize,
    /// Number of characters
    chars: usize,
}

impl FileInfo {
    /// Compute file information
    fn from_filename(filename: &Path) -> Result<Self> {
        let file: Box<dyn BufRead> = if filename.as_os_str() == "-" {
            Box::new(BufReader::new(std::io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(filename)?))
        };

        let mut reader = std::io::BufReader::new(file);
        let mut lines = 0;
        let mut words = 0;
        let mut chars = 0;

        // Fixed line count (test cases)
        loop {
            let mut line = String::new();
            let line_bytes = reader.read_line(&mut line)?;
            if line_bytes == 0 {
                break;
            }

            // This is how 'wc' counts lines
            // if line.ends_with("\n") {
            //     lines += 1;
            // }
            lines += 1;
            words += line.split_whitespace().count();
            chars += line.chars().count();
        }

        Ok(Self {
            lines,
            words,
            chars,
        })
    }

    fn format(&self, show_lines: bool, show_words: bool, show_chars: bool) -> String {
        let mut fields = Vec::new();

        for (show, value) in &[
            (show_lines, self.lines),
            (show_words, self.words),
            (show_chars, self.chars),
        ] {
            if *show {
                fields.push(format!("{:8}", value));
            }
        }

        fields.join("")
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.chars += rhs.chars;
    }
}

/// Entry point of CLI
fn main() {
    let mut cli = Cli::parse();

    if !cli.lines && !cli.words && !cli.chars {
        cli.lines = true;
        cli.words = true;
        cli.chars = true;
    }

    if cli.filenames.is_empty() {
        cli.filenames.push(PathBuf::from("-"));
    }

    let mut total = FileInfo::default();

    for filename in cli.filenames.iter() {
        let info = match FileInfo::from_filename(filename) {
            Ok(info) => info,
            Err(err) => {
                eprintln!("{}: {}", filename.display(), err);
                continue;
            }
        };

        total += info;

        println!(
            "{} {}",
            info.format(cli.lines, cli.words, cli.chars),
            filename.display()
        );
    }

    if cli.filenames.len() > 1 {
        println!("{} total", total.format(cli.lines, cli.words, cli.chars));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    // Note: we could use the `wc` command to test this, but that would
    // not work on Windows. So we test the file information directly.
    fn file_info_test(text: &str, lines: usize, words: usize, chars: usize) {
        let test_file = assert_fs::NamedTempFile::new("test.txt").unwrap();
        test_file.write_str(text).unwrap();

        let info = FileInfo::from_filename(test_file.path()).unwrap();

        assert_eq!(info.lines, lines);
        assert_eq!(info.words, words);
        assert_eq!(info.chars, chars);
    }

    #[test]
    fn file_info_empty() {
        file_info_test("", 0, 0, 0);
    }

    #[test]
    fn file_info_one_line() {
        file_info_test("Hello, world!", 1, 2, 13);
    }

    #[test]
    fn file_info_two_lines() {
        file_info_test("Hello, world!\nHello, world!", 2, 4, 27);
    }

    #[test]
    fn file_info_blank_lines() {
        file_info_test("Hello, world!\n\n\n", 3, 2, 16);
    }

    #[test]
    fn file_info_multiple_spaces() {
        file_info_test("Hello,   world!", 1, 2, 15);
    }

    #[test]
    fn file_info_unicode() {
        file_info_test("你好, 世界", 1, 2, 6);
    }

    #[test]
    fn file_info_error() {
        let info = FileInfo::from_filename(Path::new("nonexistent.txt"));

        assert!(info.is_err());
    }
}
