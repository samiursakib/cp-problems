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

// ===== Demo function =====
fn demo<R: Read, W: Write>(mut reader: R, mut writer: W) {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Scalars
    let line = lines.next().unwrap();
    let i32_number = read!(line, i32);
    let line = lines.next().unwrap();
    let i64_number = read!(line, i64);
    let line = lines.next().unwrap();
    let s = read!(line, String);

    // Tuples
    let line = lines.next().unwrap();
    let tuple = read!(line, i32, String, i64);

    // Fixed-length array
    let line = lines.next().unwrap();
    let arr_1d_fix = read!(line, [i32; 3]);

    // Variable-length array
    let line = lines.next().unwrap();
    let arr_1d_var = read!(line, [i32]);

    // // 2D array (consume rest of lines)
    let arr_2d: Vec<Vec<i32>> = read!(lines, [[i32]]);

    writeln!(writer, "i32_number={}", i32_number).unwrap();
    writeln!(writer, "i64_number={}", i64_number).unwrap();
    writeln!(writer, "string={}", s).unwrap();
    writeln!(writer, "tuple={:?}", tuple).unwrap();
    writeln!(writer, "arr_1d_fix={:?}", arr_1d_fix).unwrap();
    writeln!(writer, "arr_1d_var={:?}", arr_1d_var).unwrap();
    writeln!(writer, "arr_2d={:?}", arr_2d).unwrap();
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
        let s = read!(lines.next().unwrap(), String);
        let (mut i, mut j, mut answer) = (0, 0, 0);
        let mut answer = s.chars().map(|c| c.to_digit(10).unwrap() as i32).sum::<i32>() * 3;
        answer -= s.chars().nth(0 as usize).unwrap().to_digit(10).unwrap() as i32 + s.chars().nth((n - 1) as usize).unwrap().to_digit(10).unwrap() as i32;
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
