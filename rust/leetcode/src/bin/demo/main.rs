#[cfg(debug_assertions)]
use std::fs::{File, OpenOptions};
#[cfg(debug_assertions)]
use std::io::{self, Read, Write};

// ===== Solution Impl =====
#[cfg(debug_assertions)]
pub struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, r) in robots.iter().enumerate() {
            let pos = r + distance[i];
            let count = walls.iter().filter(|&&w| w >= pos).count() as i32;
            ans += count;

            // Optional debug print for local mode
            #[cfg(debug_assertions)]
            println!(
                "robot {}: pos = {}, added {} walls, running ans = {}",
                i, pos, count, ans
            );
        }
        ans
    }
}

// ===== Helpers for local mode =====
#[cfg(debug_assertions)]
mod local {
    use super::Solution;
    use std::fs::File;
    use std::io::{Read, Write, BufWriter};
    use std::path::Path;

    /// Parse string like "[1,2,3]" into Vec<i32>
    fn parse_vec_i32(s: &str) -> Vec<i32> {
        s.split(&[',', '[', ']', ' '] as &[_])
            .filter_map(|x| x.parse::<i32>().ok())
            .collect()
    }

    pub fn main_local() {
        let base = Path::new(file!()).parent().unwrap();
        let input_path = base.join("input.txt");
        let output_path = base.join("output.txt");

        // Read input file
        let mut input = String::new();
        File::open(input_path)
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();

        // Parse input into vectors
        let mut robots = Vec::new();
        let mut distance = Vec::new();
        let mut walls = Vec::new();

        for line in input.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let v = parse_vec_i32(value.trim());
                match key.trim() {
                    "robots" => robots = v,
                    "distance" => distance = v,
                    "walls" => walls = v,
                    _ => {}
                }
            }
        }

        // Open output file for writing
        let file = File::create(output_path).unwrap();
        let mut writer = BufWriter::new(file);

        // Call max_walls and write result
        let ans = Solution::max_walls(robots, distance, walls);
        writeln!(writer, "Final answer: {}", ans).unwrap();
    }
}

// ===== Main function =====
#[cfg(debug_assertions)]
fn main() {
    local::main_local();
}
