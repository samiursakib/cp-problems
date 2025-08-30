use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};
use std::cmp;

/// Core solve function (CodeChef-ready)
fn solve<R: Read, W: Write>(mut reader: R, mut writer: W) {
    // TODO: implement problem logic here
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();

        let result: i32 = cmp::min(a, b / 2) * 3;
        writeln!(writer, "{}", result).unwrap();
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        let base = std::path::Path::new(file!())
            .parent()
            .unwrap();
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
