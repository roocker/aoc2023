use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader},
    usize,
};

fn day01a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // todo #rev
            // let (first, last) = find_first_last(line);
        })
        .sum()
}

fn day01b(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (first, last) = find_first_last(line);
            format!("{}{}", first, last).parse::<usize>().unwrap();
        })
        .sum()
}

fn main() {
    let filecontent = fs::read_to_string("input/day01.txt").unwrap();
    println!("result of day01a: {}", day01a(&filecontent));
    println!("result of day01b: {}", day01b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "";

    #[test]
    fn test_day01a() {
        assert_eq!(281, day01a(TEST_STRINGS))
    }
    #[test]
    fn test_day01b() {
        assert_eq!(281, day01b(TEST_STRINGS))
    }
}
