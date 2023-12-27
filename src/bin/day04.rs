use std::fs;
fn winning_nums(input: &str) -> Vec<Vec<u8>> {
    let nums_won = input
        .lines()
        .map(|line| {
            let (_id, nums) = line.split_once(':').unwrap();
            let (winning_nums, nums) = nums.split_once('|').unwrap();

            let winning_nums = winning_nums
                .split_whitespace()
                .map(|s| s.parse().expect("should be parseable"))
                .collect::<Vec<u8>>();

            let nums = nums
                .split_whitespace()
                .map(|s| s.parse().expect("should be parseable"))
                .collect::<Vec<u8>>();

            // println!("{id}: {:?} | {:?}", winning_nums, nums);

            let nums_won = nums
                .iter()
                .filter_map(|num| {
                    if winning_nums.contains(num) {
                        Some(*num)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8>>();
            nums_won
        })
        .collect::<Vec<_>>();
    nums_won
}

fn day04a(input: &str) -> usize {
    let nums_won = winning_nums(input);

    let score: usize = nums_won
        .iter()
        .map(|nums| {
            if !nums.is_empty() {
                2usize.pow(nums.len() as u32 - 1)
            } else {
                0
            }
        })
        .sum();
    score
}

fn getscore(id: usize, cards: &Vec<usize>, counter: &mut usize) -> usize {
    if cards[id] > 0 {
        for i in id + 1..=id + cards[id] {
            *counter += 1;
            getscore(i, cards, counter);
        }
    }
    cards[id]
}

fn day04b(input: &str) -> usize {
    let nums_won = winning_nums(input);

    let mut counter = 0;
    let mut cards = Vec::new();

    nums_won.iter().for_each(|res| cards.push(res.len()));

    // let cards = vec![4, 2, 2, 1, 0, 0];
    // println!("cards vec{:?}", cards);

    for i in 0..cards.len() {
        getscore(i, &cards, &mut counter);
        counter += 1;
    }

    counter
}

fn main() {
    let filecontent = fs::read_to_string("input/day04.txt").unwrap();
    println!("result of day04a: {}", day04a(&filecontent));
    println!("result of day04b: {}", day04b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day04a() {
        assert_eq!(13, day04a(TEST_STRINGS))
    }
    #[test]
    fn test_day04b() {
        assert_eq!(30, day04b(TEST_STRINGS))
    }
}
