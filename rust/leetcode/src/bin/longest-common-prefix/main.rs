use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::{Mutex, OnceLock};

pub struct Solution;
fn print_ref(x: &i32) -> &i32 {
    println!("{}", x);
    x
}

// ===== LeetCode struct =====
// TODO: implement problem logic here
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let (mut ans, mut cur_index, mut sorted_words) = (String::new(), 0, strs.clone());
        sorted_words.sort();
        let min_count = std::cmp::min(sorted_words[0].len(), sorted_words[sorted_words.len() - 1].len());
        while cur_index < min_count {
            let first_char = sorted_words[0].chars().nth(cur_index).unwrap();
            if first_char == sorted_words[sorted_words.len() - 1].chars().nth(cur_index).unwrap() {
                ans.push(first_char);
            } else {
                return ans;
            }
            cur_index += 1;
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
    ($line:expr, String) => { $line.split_whitespace().next().unwrap().to_string() };
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
            let strs = read!(lines.next().unwrap(), [String]);
            let ans = Solution::longest_common_prefix(strs);
            writeln!(writer, "{:?}", ans).unwrap();
        }
    }
}
