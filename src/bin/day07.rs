use std::{collections::HashMap, fs};

fn read_hands(input: &str) -> Vec<(&str, usize)> {
    let mut hands = Vec::new();

    let _ = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            hands.push((hand, bid.parse().unwrap()));
        })
        .collect::<Vec<_>>();

    hands
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

    // println!("hand:{} counts {:?} - points: {}", hand, rankings, points);
    pairs + three_of_a_kind + four_of_a_kind + five_of_a_kind + full_house
}

fn day07a(input: &str) -> usize {
    let mut hands = read_hands(input);

    hands.sort_by(|(a, _), (b, _)| {
        let rank_a = rank_hand(a);
        let rank_b = rank_hand(b);

        if rank_a == rank_b {
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
        .map(|(i, (_, bid))| {
            // println!("{} - {}:{}", i, card, (i + 1) * bid);
            bid * (i + 1)
        })
        .sum()
}

fn rank_hand_j_joker(hand: &str) -> u32 {
    let cards = "23456789TJQKA";
    let mut rankings: HashMap<char, u32> = HashMap::new();
    let mut joker = 0;
    for card in hand.chars() {
        *rankings.entry(card).or_insert(0) += 1;
        if card == 'J' {
            joker += 1;
            //remove j from rankings
            rankings.remove_entry(&'J');
        }
    }

    if joker > 0 {
        for _ in 0..joker {
            let mut _highcard = 'J';
            let (most_of_this_card_key, _) = rankings.iter().max_by_key(|(_, v)| *v).unwrap_or({
                _highcard = cards
                    .chars()
                    .filter(|c| !rankings.contains_key(c))
                    .last()
                    .unwrap();

                (&_highcard, &2)
            });

            rankings
                .entry(*most_of_this_card_key)
                .and_modify(|v| *v += 1);
        }
    }

    /* println!(
        // "hand: {:?} rankings: {:?} , joker: {}",
        hand, rankings, joker
    ); */

    let mut pairs = 0;
    let mut three_of_a_kind = 0;
    let mut four_of_a_kind = 0;
    let mut five_of_a_kind = 0;
    let mut full_house = 0;
    let mut five_of_a_kind_joker = 0;

    if joker == 5 {
        five_of_a_kind_joker += 6;
    }

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

    // println!("hand:{} counts {:?} - points: {}", hand, rankings, points);
    pairs + three_of_a_kind + four_of_a_kind + five_of_a_kind + full_house + five_of_a_kind_joker
}

fn day07b(input: &str) -> usize {
    let mut hands = read_hands(input);

    hands.sort_by(|(a, _), (b, _)| {
        let rank_a = rank_hand_j_joker(a);
        let rank_b = rank_hand_j_joker(b);

        if rank_a == rank_b {
            let cards = "J23456789TQKA";
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
        .map(|(i, (_, bid))| {
            // println!("{} - {}:{}", i, card, (i + 1) * bid);
            bid * (i + 1)
        })
        .sum()
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
    println!("---");
    println!("PART A:");
    println!("result of day07a: {}", day07a(&filecontent));
    // 252295678
    println!("---");
    println!("PART B:");
    // 250577259
    println!("result of day07b: {}", day07b(&filecontent));
    println!("---");
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
        assert_eq!(5905, day07b(TEST_STRINGS))
    }
}
