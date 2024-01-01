use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{self, BufRead, BufReader},
    usize,
};

fn day07a(input: &str) -> usize {
    let mut hands = Vec::new();
    // let mut hands = BTreeMap::new();

    let _ = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            hands.push((hand, bid.parse().unwrap()));
            // hands.insert(hand, bid);
        })
        .collect::<Vec<_>>();

    // println!("hands: {:?} ", hands);

    /* const CARDS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]; */

    hands.sort_by(|(a, _), (b, _)| {
        let rank_a = rank_hand(a);
        let rank_b = rank_hand(b);

        if rank_a == rank_b {
            // let cards = "AKQJT98765432";
            // println!("a {} & b {} = {},{} have equal score", a, b, rank_a, rank_b);
            let cards = "23456789TJQKA";
            let a_chars: Vec<char> = a.chars().collect();
            let b_chars: Vec<char> = b.chars().collect();

            for i in 0..a_chars.len() {
                let a_index = cards.chars().position(|c| c == a_chars[i]).unwrap();
                let b_index = cards.chars().position(|c| c == b_chars[i]).unwrap();
                if a_index != b_index {
                    return a_index.cmp(&b_index);
                }
            }
            std::cmp::Ordering::Equal
        } else {
            rank_a.cmp(&rank_b)
        }
    });

    // println!("sorted hands: {:?} ", hands);

    hands
        .iter()
        .enumerate()
        .map(|(i, (card, bid))| {
            println!("{} - {}:{}", i, card, (i + 1) * bid);
            bid * (i + 1)
        })
        .sum()
}
fn rank_hand(hand: &str) -> u32 {
    let mut rankings: HashMap<char, u32> = HashMap::new();
    for card in hand.chars() {
        *rankings.entry(card).or_insert(0) += 1;
    }

    let mut pairs = 0;
    let mut three_of_a_kind = 0;
    let mut four_of_a_kind = 0;
    let mut five_of_a_kind = 0;
    let mut full_house = 0;

    for rank in rankings.values() {
        match rank {
            2 => pairs += 1,
            3 => three_of_a_kind += 3,
            4 => four_of_a_kind += 5,
            5 => five_of_a_kind += 6,
            _ => (),
        }
    }

    if three_of_a_kind > 0 && pairs > 0 {
        full_house += 4;
        three_of_a_kind = 0;
        pairs = 0;
    }

    let points = pairs + three_of_a_kind + four_of_a_kind + five_of_a_kind + full_house;
    // println!("hand:{} counts {:?} - points: {}", hand, rankings, points);
    points
}

fn day07b(input: &str) -> usize {
    todo!();
}

fn main() {
    /* 11111 111
    12111 111
    12211 111
    77888 111
    77788 111 */
    /* const filecontent: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"; */
    let filecontent = fs::read_to_string("input/day07.txt").unwrap();
    println!("result of day07a: {}", day07a(&filecontent));
    // 252295678

    // println!("result of day07b: {}", day07b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_day07a() {
        assert_eq!(6440, day07a(TEST_STRINGS))
    }
    #[test]
    fn test_day07b() {
        assert_eq!(281, day07b(TEST_STRINGS))
    }
}
