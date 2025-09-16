use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::{Mutex, OnceLock};

pub struct Solution;

// ===== LeetCode struct =====
// TODO: implement problem logic here
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        ans.append(nums.as_mut());
        ans
    }
}


// ===== Universal read macro (line-aware) =====
macro_rules! read {    
    ($lines:expr, [[$t:ty]]) => {{ $lines.map(|line| line.split_whitespace().map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>()).collect::<Vec<Vec<$t>>>() }};
    ($lines:expr, [[String]]) => {{ $lines.map(|line| line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>() }};
    ($line:expr, [$t:ty; $n:expr]) => {{ $line.split_whitespace().take($n).map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>() }};
    ($line:expr, [String; $n:expr]) => {{ $line.split_whitespace().take($n).map(|s| s.to_string()).collect::<Vec<String>>() }};
    ($line:expr, [$t:ty]) => {{ $line.split_whitespace().map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>() }};
    ($line:expr, [String]) => {{ $line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() }};
    ($line:expr, String) => { $line.split_whitespace().next().unwrap().to_string() };
    ($line:expr, $t:ty) => { $line.split_whitespace().next().unwrap().parse::<$t>().unwrap() };
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
            let nums = read!(lines.next().unwrap(), [i32]);
            let ans = Solution::get_concatenation(nums);
            writeln!(writer, "{:?}", ans).unwrap();
        }
    }
}
