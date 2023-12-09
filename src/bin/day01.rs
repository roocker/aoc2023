use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader},
    usize,
};

fn find_first_last(line: &str) -> (u8, u8) {
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

    let mut allnumbers: Vec<(usize, u8)> = Vec::new();

    for (str, val) in numbers {
        let mut start_i = 0;

        while let Some(i) = line[start_i..].find(str) {
            let actual_i = start_i + i;
            // println!("word '{}' at i {}", val, actual_i);
            allnumbers.push((actual_i, val));

            start_i = actual_i + 1;
        }
    }

    for (index, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            let digit = char.to_digit(10).unwrap() as u8;
            // println!("digit: {} , at i: {} ", digit, index);
            allnumbers.push((index, digit));
        }
    }

    let (_, first) = allnumbers.iter().min_by_key(|x| x.0).unwrap();
    let (_, last) = allnumbers.iter().max_by_key(|x| x.0).unwrap();

    println!("input string: {} | first: {} | last {}", line, first, last);
    (*first, *last)
}

fn day01b(input_strings: &str) -> usize {
    let mut sum: usize = 0;

    for line in input_strings.lines() {
        let (first, last) = find_first_last(line);
        let linenumber: usize = format!("{}{}", first, last).parse().unwrap();
        sum += linenumber
    }
    sum
}

fn main() {
    // const TEST_STRINGS: &str = "aeightseven651\nseven90nine0\n";
    // println!("result of day01b: {}", day01b(TEST_STRINGS));
    let filecontent = fs::read_to_string("input/day01.txt").unwrap();
    println!("result of day01b: {}", day01b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn teststring() {
        assert_eq!(281, day01b(TEST_STRINGS))
    }
}
