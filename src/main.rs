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

fn find_calibration<F: BufRead>(file: F) /* -> usize */
{
    let file = file.lines();
    let mut linenumber: usize = 0;
    let mut sum: usize = 0;

    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
        ("null", 0),
    ];

    /* let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero", "null",
    ]; */

    for line in file {
        let line = line.unwrap();
        println!("\nstring: {}", line);

        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;

        let mut first_index: Option<usize> = None;
        let mut last_index: Option<usize> = None;

        for (str, val) in numbers {
            let mut start_i = 0;

            while let Some(i) = line[start_i..].find(str) {
                let actual_i = start_i + i;
                println!("found '{}' at i {}", val, actual_i);

                if first_index.is_none() || actual_i < first_index.unwrap() {
                    first_index = Some(actual_i);
                    first = Some(val);
                }

                if last_index.is_none() || actual_i > last_index.unwrap() {
                    last_index = Some(actual_i);
                    last = Some(val);
                }

                start_i = actual_i + 1;
            }
        }

        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let n = c.to_digit(10).unwrap() as u8;
                if first.is_none() {
                    first = Some(n);
                    println!("found# '{}' at i {}", c, i);
                    first_index = Some(i)
                }
                if last.is_none() {
                    last = Some(n);
                    println!("found# '{}' at i {}", c, i);
                    last_index = Some(i)
                }
            }
        }

        if let (Some(first), Some(last)) = (first, last) {
            println!("First number: {}", first);
            println!("Last number: {}", last);
            linenumber = format!("{}{}", first, last).parse().unwrap();
            sum += linenumber
        }
        println!("Linenumber: {}", linenumber);
    }

    println!("------------");
    println!("sum: {}", sum);
}

fn main() {
    if let Ok(file) = read_file("testinput.txt") {
        find_calibration(file)
    } else {
        {
            eprintln!("error reading file")
        }
    }
}
