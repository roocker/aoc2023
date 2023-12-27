use itertools::Itertools;
use std::{collections::BTreeMap, fs, usize};

#[derive(Debug)]

enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

fn create_map(input: &str) -> BTreeMap<(i32, i32), Value> {
    //create the entire map of everything
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    // (x as i32, y as i32),
                    (y as i32, x as i32),
                    match c {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("should be a number"))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Value>>()
    // dbg!(map);
}

fn parse_numbers(map: &BTreeMap<(i32, i32), Value>) -> Vec<Vec<((i32, i32), u32)>> {
    //rebuilding numbers from digits in the map
    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("should exist");
                                last.push(((*x, *y), *num))
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("shouldn't happen"),
                    }
                }
                None => numbers.push(vec![((*x, *y), *num)]),
            }
            // println!("{x}, {y}");
        }
    }
    numbers
    // dbg!(numbers);
}

fn day03a(input: &str) -> u32 {
    let map = create_map(input);
    let numbers = parse_numbers(&map);

    let mut total = 0;
    for num_list in numbers {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();

        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions
                    .iter()
                    .map(|outer_pos| (outer_pos.0 + pos.1, outer_pos.1 + pos.0))
            })
            .filter(|num| !num_positions.contains(num))
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);
        let is_part_number = pos_to_check.iter().any(|pos| {
            let value = map.get(pos);
            // matches!(value, Some(Value::Symbol(_)))
            if let Some(Value::Symbol(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        }

        // println!("{n} - {is_part_number}");
        // dbg!(is_part_number, n);
    }
    total

    // println!("{:?}", map);
}

fn day03b(input: &str) -> usize {
    let map = create_map(input);
    let numbers = parse_numbers(&map);

    let mut total = 0;
    for symbol in map
        .iter()
        .filter(|(_, value)| matches!(value, Value::Symbol('*')))
    {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| (outer_pos.0 + symbol.0 .1, outer_pos.1 + symbol.0 .0))
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);
        let mut index_of_numbers = vec![];

        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list
                    .iter()
                    .find(|(num_pos, _)| num_pos == &pos)
                    .is_some()
                {
                    index_of_numbers.push(i);
                }
            }
        }

        let is_gear = index_of_numbers.iter().unique().count() == 2;

        if is_gear {
            // println!(" ");
            total += index_of_numbers
                .iter()
                .unique()
                /* .inspect(|i| {
                    println!("{i}");
                }) */
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .unique()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>()
        }
    }
    total
}

fn main() {
    let filecontent = fs::read_to_string("input/day03.txt").unwrap();

    println!("result of day03a: {:?}", day03a(&filecontent));
    println!("result of day03b: {}", day03b(&filecontent));
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
    #[test]
    fn test_day03b() {
        assert_eq!(467835, day03b(TEST_STRINGS))
    }
}
