use std::fs;

fn main() {
    let filecontent = fs::read_to_string("input.txt").unwrap();

    let mut sum: usize = 0;

    for line in filecontent.lines() {
        let games = parse_games(line);
        let (id, result) = test_game(game);
        println!("{} | {}", line, result);
        if result {
            sum += id
        }
    }
}

// red green blue enum
// fn main
// fn parse_games {}
// fn test_game()

mod tests {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    const TESTINPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn name() {}
}
