use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::{Mutex, OnceLock};

pub struct Solution;

// ===== LeetCode struct =====
// TODO: implement problem logic here
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = word1
            .chars()
            .zip(word2.chars())
            .map(|(a, b)| a.to_string() + &b.to_string())
            .collect::<Vec<String>>()
            .join("");

        if word1.len() > ans.len() / 2 {
            ans += &word1[ans.len() / 2..];
        }
        if word2.len() > ans.len() / 2 {
            ans += &word2[ans.len() / 2..];
        }
        ans
    }
}

// ===== Universal read macro (line-aware) =====
macro_rules! read {
    // 2D array (consume multiple lines)
    ($lines:expr, [[$t:ty]]) => {{ $lines.map(|line| line.split_whitespace().map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>()).collect::<Vec<Vec<$t>>>() }};
    ($lines:expr, [[String]]) => {{ $lines.map(|line| line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>() }};
    // Fixed-length array from a single line
    ($line:expr, [$t:ty; $n:expr]) => {{ $line.split_whitespace().take($n).map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>() }};
    ($line:expr, [String; $n:expr]) => {{ $line.split_whitespace().take($n).map(|s| s.to_string()).collect::<Vec<String>>() }};
    // Variable-length array from a single line
    ($line:expr, [$t:ty]) => {{ $line.split_whitespace().map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>() }};
    ($line:expr, [String]) => {{ $line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() }};
    // Single values
    ($line:expr, String) => { $line.to_string() };
    ($line:expr, $t:ty) => { $line.split_whitespace().next().unwrap().parse::<$t>().unwrap() };
    // Multiple values on a single line
    ($line:expr, String, $($rest:ty),+) => {{ let mut it = $line.split_whitespace(); (it.next().unwrap().to_string(), $( it.next().unwrap().parse::<$rest>().unwrap() ),+) }};
    ($line:expr, $t:ty, $($rest:ty),+) => {{ let mut it = $line.split_whitespace(); (it.next().unwrap().parse::<$t>().unwrap(), $( it.next().unwrap().parse::<$rest>().unwrap() ),+) }};
}

// ===== Global Writer for Debugging =====
#[cfg(debug_assertions)]
static WRITER: OnceLock<Mutex<Box<dyn Write + Send>>> = OnceLock::new();

#[cfg(debug_assertions)]
fn init_writer() {
    WRITER.get_or_init(|| {
        let base = std::path::Path::new(file!()).parent().unwrap();
        let output_path = base.join("output.txt");
        let writer = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_path)
            .unwrap();
        Mutex::new(Box::new(writer) as Box<dyn Write + Send>)
    });
}

// ===== main =====
fn main() {
    #[cfg(debug_assertions)]
    {
        use std::fs::File;
        init_writer();

        let base = std::path::Path::new(file!()).parent().unwrap();
        let input_path = base.join("input.txt");

        let mut input = String::new();
        File::open(input_path).unwrap().read_to_string(&mut input).unwrap();
        let mut lines = input.lines();
        let mut writer = WRITER.get().unwrap().lock().unwrap();

        let test_cases = read!(lines.next().unwrap(), i32);
        for _ in 0..test_cases {
            // iterate each test case
            let word1 = read!(lines.next().unwrap(), String);
            let word2 = read!(lines.next().unwrap(), String);
            let ans = Solution::merge_alternately(word1, word2);
            writeln!(writer, "{:?}", ans).unwrap();
        }
    }
}
