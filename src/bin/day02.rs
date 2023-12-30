// red green blue enum
// fn main
// fn day02a
// fn parse_games {}
// fn test_game()
use std::{fs, usize};

#[derive(Debug)]
struct Game {
    id: usize,
    // color: (r, g, b)
    colors: Vec<(usize, usize, usize)>,
    possibles: Vec<bool>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (id, rest) = line.split_once(':').unwrap();
        let (_, id) = id.split_once(' ').unwrap();
        let id: usize = id.parse().unwrap();

        let sets: Vec<_> = rest.split(';').collect();
        // let sets = sets.split(',');
        // println!(" SETS:{:?}", sets);

        let colors: Vec<_> = sets
            .iter()
            .map(|set| {
                // println!(" SET:{:?}", set);
                let (mut red, mut green, mut blue) = (0, 0, 0);
                set.split(',').fold((red, green, blue), |mut acc, part| {
                    let (val, col) = part.trim().split_once(' ').unwrap();
                    let val: usize = val.parse().unwrap();
                    match col {
                        "red" => red = val,
                        "green" => green = val,
                        "blue" => blue = val,
                        _ => {
                            red = 0;
                            green = 0;
                            blue = 0;
                        }
                    };
                    (red, green, blue)
                })
            })
            .collect();

        // println!("{:?}", colors);

        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        let possibles: Vec<_> = colors
            .iter()
            .map(|color| color.0 <= max_red && color.1 <= max_green && color.2 <= max_blue)
            .collect();

        Self {
            id,
            colors,
            possibles,
        }
    }

    fn score_game(game: Game) -> usize {
        let possible = !game.possibles.contains(&false);
        if possible {
            game.id
        } else {
            0
        }
    }

    fn min_count(game: &Game) -> usize {
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;
        // let (mut min_r, mut min_g, mut min_b) = (0, 0, 0);

        game.colors.iter().for_each(|col| {
            // println!("min ({}, {}, {})", min_r, min_b, min_b);
            // println!("col {:?}", col);
            if col.0 >= min_r {
                min_r = col.0
            }
            if col.1 > min_g {
                min_g = col.1;
            }
            if col.2 > min_b {
                min_b = col.2;
            }
            /* if (col.0, col.1, col.2) > (min_r, min_g, min_b) {
                min_r = col.0;
                min_g = col.1;
                min_b = col.2;
            } */
        });

        // println!("MIN : {},{},{}", min_r, min_g, min_b);

        min_r * min_g * min_b
    }
}

fn day02a(input: &str) -> usize {
    let scores = input
        .lines()
        .map(|line| {
            let game = Game::parse(line);
            // println!("{:?}", game);
            Game::score_game(game)
        })
        .sum();

    scores
}

fn day02b(input: &str) -> usize {
    println!("----------");
    let min_set_sum = input
        .lines()
        .map(|line| {
            let game = Game::parse(line);
            let power = Game::min_count(&game);
            // println!("{:?} - {}", game, power);
            power
        })
        .sum();

    min_set_sum
}

fn main() {
    /*
        let filecontent: String = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
    */

    let filecontent = fs::read_to_string("./input/day02.txt").unwrap();

    println!("day02a result: {}", day02a(&filecontent));
    println!("day02b result: {}", day02b(&filecontent));
}

mod tests {
    use super::*;
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    const TESTINPUT_DAY02A: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_day02a() {
        assert_eq!(8, day02a(TESTINPUT_DAY02A));
    }

    #[test]
    fn test_day02b() {
        assert_eq!(2286, day02b(TESTINPUT_DAY02A));
    }
}
