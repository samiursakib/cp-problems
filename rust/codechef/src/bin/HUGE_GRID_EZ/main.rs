use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};

// ===== Universal read macro =====
macro_rules! read {
    // Single values
    ($it:expr, String) => { $it.next().unwrap().to_string() };
    ($it:expr, $t:ty) => { $it.next().unwrap().parse::<$t>().unwrap() };

    // 1D fixed-length array
    ($it:expr, [$t:ty; $n:expr]) => { (0..$n).map(|_| $it.next().unwrap().parse::<$t>().unwrap()).collect::<Vec<$t>>() };
    ($it:expr, [String; $n:expr]) => { (0..$n).map(|_| $it.next().unwrap().to_string()).collect::<Vec<String>>() };

    // 1D variable-length array (consume all remaining tokens)
    ($it:expr, [$t:ty]) => { $it.map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>() };
    ($it:expr, [String]) => { $it.map(|s| s.to_string()).collect::<Vec<String>>() };

    // 2D variable-length array: each "row" is a whitespace-separated string
    ($it:expr, [[$t:ty]]) => { $it.map(|row| row.split_whitespace().map(|s| s.parse::<$t>().unwrap()).collect::<Vec<$t>>()).collect::<Vec<Vec<$t>>>() };
    ($it:expr, [[String]]) => { $it.map(|row| row.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>() };

    // Multiple values at once
    ($it:expr, String, $($rest:ty),+) => { ($it.next().unwrap().to_string(), $( $it.next().unwrap().parse::<$rest>().unwrap() ),+ ) };
    ($it:expr, $t:ty, $($rest:ty),+) => { ($it.next().unwrap().parse::<$t>().unwrap(), $( $it.next().unwrap().parse::<$rest>().unwrap() ),+ ) };
}

// ===== Demo function =====
fn demo<R: Read, W: Write>(mut reader: R, mut writer: W) {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a = read!(it, i32);
    let b = read!(it, i64);
    let s = read!(it, String);
    // let arr_fixed = read!(it, [i32; 3]);
    // let arr_var = read!(it, [i32]);
    let tuple = read!(it, i32, String, i64);

    writeln!(writer, "a={} b={} s={}", a, b, s).unwrap();
    // writeln!(writer, "arr_fixed={:?}", arr_fixed).unwrap();
    // writeln!(writer, "arr_var={:?}", arr_var).unwrap();
    writeln!(writer, "tuple={:?}", tuple).unwrap();
}

// ===== Core solve function =====
fn solve<R: Read, W: Write>(reader: R, writer: W) {
    demo(reader, writer);
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
