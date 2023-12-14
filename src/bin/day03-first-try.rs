use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader},
    usize,
};

// 3 lines
// take 3 lines,
// find special chars and index
// find numbers and index
// filter numbers "near" specialchars

// map special chars with x/y coords (linenumber) -> Symbol(x: u8, y: u8); Vec<Symbol<x,y,val>, Symbol<x,y,val, ....>
// find numbers -> Number(x: u8, y: u8, val: u16) -> Vec<Number<
// find numbers "near" symbols

// if Symbol.x -1..Symbol.x+1 == Number.x - 1 .. Number.x+Number.val.length()+1

// same line before & after:
// [Number.x-1 ..= Number.x+Number.val.length()+1]

// next prev y and before after x
// [Number.y-1 ..= Number.y+1] &&
// [Number.x-1 ..= Number.x+Number.val.length()+1]

#[derive(Debug)]
struct Symbol {
    x: u8,
    y: u8,
    val: char,
}
impl Symbol {
    fn new(x: u8, y: u8, val: char) -> Self {
        Self { x, y, val }
    }
}

#[derive(Debug)]
struct Number {
    x: u8,
    y: u8,
    val: u16,
}
impl Number {
    fn new(x: u8, y: u8, val: u16) -> Self {
        Self { x, y, val }
    }
}

const SYMBOLS: [char; 10] = ['*', '#', '=', '+', '-', '@', '/', '&', '%', '$'];

fn read_symbols(linenumber: usize, line: &str) -> Vec<Symbol> {
    // println!("{}", line);
    let symbols: Vec<Symbol> = line
        .chars()
        .enumerate()
        .filter(|(_, c)| SYMBOLS.contains(c))
        .map(|(index, c)| Symbol::new(index as u8, linenumber as u8, c))
        .collect();
    // println!("sym{:?}", symbols);
    symbols
}

fn read_numbers(linenumber: usize, line: &str) -> Vec<Number> {
    /* let numbers: Vec<_> = line
        .split(|c: char| !c.is_ascii_digit())
        .enumerate()
        .filter(|(_, s)| !s.is_empty())
        .map(|(index, num_str)| {
            // index as u8 + num_str.len() as u8;
            Number::new(x, linenumber as u8, num_str.parse().unwrap())
        })
        .collect();
    */

    // let testnum = Number::new(0, 0, 0);
    // println!("!!!!{:?}", testnum);

    let mut line = line.to_string();
    line.push(' '); // Add a space at the end

    let mut start_index = 0;
    let mut end_index = 0;

    let mut numbers: Vec<Number> = Vec::new();
    line.chars().enumerate().for_each(|(i, c)| {
        if !c.is_ascii_digit() {
            if start_index != end_index {
                let num_str = &line[start_index..end_index];
                let number = Number::new(
                    start_index as u8,
                    linenumber as u8,
                    num_str.parse().unwrap(),
                );
                numbers.push(number)
            }
            start_index = i + 1;
        }
        end_index = i + 1;
    });

    numbers
}
fn count_digits(n: u16) -> u8 {
    n.to_string().len() as u8
}
fn numbers_near_symbols(symbols: Vec<Symbol>, numbers: Vec<Number>) -> Vec<usize> {
    // [Number.y-1 ..= Number.y+1] &&
    // [Number.x-1 ..= Number.x+Number.val.length()+1]

    // compare to symbols.x symbols.y

    let mut result: Vec<usize> = Vec::new();

    // test ok, but input.txt wrong
    for num in &numbers {
        for sym in &symbols {
            if ((num.x != 0 && num.x - 1 == sym.x) || (num.x + 1 == sym.x) || (num.x == sym.x))
                && ((num.y != 0 && num.y - 1 == sym.y) || (num.y + 1 == sym.y) || (num.y == sym.y))
                || ((num.x + count_digits(num.val) == sym.x)
                    || (num.x + count_digits(num.val) + 1 == sym.x)
                    || (num.x + count_digits(num.val) - 1 == sym.x))
                    && ((num.y != 0 && num.y - 1 == sym.y)
                        || (num.y + 1 == sym.y)
                        || (num.y == sym.y))
            // && ((num.y != 0 && num.y - 1 == sym.y)
            //     || (num.y + 1 == sym.y)
            //     || (num.y == sym.y))
            {
                result.push(num.val as usize);
                break;
            }
        }
    }

    result

    // let numbers_near_symbols = numbers;
    //
    // numbers_near_symbols
    //     .iter()
    //     .map(|n| n.val as usize)
    //     .collect::<Vec<usize>>()
}

fn day03a(input: &str) -> usize {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    input.lines().enumerate().for_each(|(index, line)| {
        symbols.push(read_symbols(index, line));
        // println!("heeast{:?}", read_symbols(index, line));
        numbers.push(read_numbers(index, line));
    });

    let symbols: Vec<Symbol> = symbols.into_iter().flat_map(|s| s).collect();

    let numbers: Vec<Number> = numbers.into_iter().flat_map(|n| n).collect();

    // println!("Symbols: {:#?}", symbols);
    // println!("Numbers: {:#?}", numbers);
    let result = numbers_near_symbols(symbols, numbers);

    // println!("RESULTS: {:?}", result);
    result.into_iter().sum()
}

fn day03b(input: &str) -> usize {
    todo!()
}

fn main() {
    /*
        let filecontent: &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";
    */

    // ist  518022
    // soll 515900?!?!?
    // oder 514969
    // 2122

    let filecontent = fs::read_to_string("input/day03.txt").unwrap();
    println!("result of day03a: {}", day03a(&filecontent));
    println!("----------------------");
    // println!("result of day03b: {}", day03b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_day03a() {
        assert_eq!(4361, day03a(TEST_STRINGS))
    }
    fn test_day03b() {
        assert_eq!(281, day03b(TEST_STRINGS))
    }
}
