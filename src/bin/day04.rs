use std::{
    fs,
    io::{self, BufRead, BufReader},
    usize,
};

fn read_file(path: &str) -> io::Result<BufReader<fs::File>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}
/*
1. read input
2. for line in input.lines()
3. let mut count = 0;
4. sepearte | numbers into two vecs:
Vec<u8>
let winning = Vec::new();
let mynumbers = Vec::new()

if winning.contains(mynumbers) {
if !count.is_null{
count =+1
} else count *= 2

*/

fn day04<F: BufRead>(file: F) {
    for line in file.lines() {
        line.map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            println!("{}:::{}", left, right);
        });

        // println!("line {}", line.unwrap());
    }
}

fn main() {
    if let Ok(file) = read_file("testinput.txt") {
        day04(file)
    } else {
        {
            eprintln!("error reading file")
        }
    }
}
