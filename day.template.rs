use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader},
    usize,
};

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
