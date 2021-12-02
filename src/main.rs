use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1() -> i32 {
    0
}

fn p2() -> i32 {
    0
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let line_vec: Vec<&str> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(s) = line {
                // Process each line...
            }
        }
    }
    println!("Part 1: {}\nPart 2:{}", p1(), p2());
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
