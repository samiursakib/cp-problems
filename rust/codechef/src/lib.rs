use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

/// Helper: read entire file into a string
fn read_file(filename: &PathBuf) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

/// Resolve the problem folder (src/bin/<name>)
fn problem_dir() -> io::Result<PathBuf> {
    let exe_path = env::current_exe()?; // e.g. target/debug/demo
    let name = exe_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut dir = env::current_dir()?; // project root (where Cargo.toml is)
    dir.push("src/bin");
    dir.push(&name);
    Ok(dir)
}

/// InputReader struct
pub struct InputReader {
    tokens: Vec<String>,
    pos: usize,
}

impl InputReader {
    /// Create from a string
    pub fn from_str(input: &str) -> Self {
        let tokens = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        Self { tokens, pos: 0 }
    }

    /// Create from local input.txt
    pub fn from_local_file() -> io::Result<Self> {
        let mut path = problem_dir()?;
        path.push("input.txt");

        let input = read_file(&path)?;
        Ok(Self::from_str(&input))
    }

    /// Read next token as T
    pub fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token.parse::<T>().expect("Failed to parse input")
    }

    /// Read next n tokens as Vec<T>
    pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.next()).collect()
    }
}

/// OutputWriter struct
pub struct OutputWriter {
    file: File,
}

impl OutputWriter {
    pub fn to_local_file() -> io::Result<Self> {
        let mut path = problem_dir()?;
        path.push("output.txt");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        Ok(Self { file })
    }

    pub fn writeln<S: AsRef<str>>(&mut self, s: S) -> io::Result<()> {
        writeln!(self.file, "{}", s.as_ref())
    }
}
