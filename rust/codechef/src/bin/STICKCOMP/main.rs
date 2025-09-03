use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};

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

// ===== Core solve function =====
fn solve<R: Read, W: Write>(mut reader: R, mut writer: W) {
    // TODO: implement problem logic here
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = read!(lines.next().unwrap(), i32);
    for _ in 0..t {
        let n = read!(lines.next().unwrap(), i32);
        let arr = read!(lines.next().unwrap(), [i32]);
        let mut answer = 1;
        let mut currentMax = arr[0];
        for i in 1..n {
            if arr[i as usize] > currentMax {
                currentMax = arr[i as usize];
                answer = i + 1;
            }
        }
        writeln!(writer, "{}", answer).unwrap();
    }
}

// ===== main =====
fn main() {
    #[cfg(debug_assertions)]
    {
        let base = std::path::Path::new(file!()).parent().unwrap();
        let input_path = base.join("input.txt");
        let output_path = base.join("output.txt");

        let reader = File::open(input_path).unwrap();
        let writer = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_path)
            .unwrap();

        solve(reader, writer);
    }

    #[cfg(not(debug_assertions))]
    {
        let stdin = io::stdin();
        let stdout = io::stdout();
        solve(stdin.lock(), stdout.lock());
    }
}

// ===== Universal read macro =====
// let mut input = String::new();
// reader.read_to_string(&mut input).unwrap();
// let mut lines = input.lines();
// Scalars
// let i32_number = read!(line, i32);
// let i64_number = read!(line, i64);
// let s = read!(line, String);
// Tuples
// let tuple = read!(line, i32, String, i64);
// Fixed-length array
// let arr_1d_fix = read!(line, [i32; 3]);
// Variable-length array
// let arr_1d_var = read!(line, [i32]);
// 2D array (consume rest of lines)
// let arr_2d: Vec<Vec<i32>> = read!(lines, [[i32]]);
