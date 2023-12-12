use std::fs;

use hashbrown::HashMap;

fn find_symbol(lines: &[&[u8]], r: usize, c: usize) -> Option<(usize, usize, char)> {
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (rr, cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
        let Some(&s) = lines.get(rr).and_then(|l| l.get(cc)) else {
            continue;
        };
        if s != b'.' && !s.is_ascii_digit() {
            return Some((cc, rr, s as char));
        }
    }
    None
}
// #[aoc::main(03)]
fn main() {
    let input = fs::read_to_string("input/day03.txt").unwrap();
    let lines = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut symbols = HashMap::new();
    for (r, l) in lines.iter().enumerate() {
        let mut c = 0;
        while c < l.len() {
            let (start, mut symbol) = (c, None);
            while c < l.len() && l[c].is_ascii_digit() {
                symbol = symbol.or_else(|| find_symbol(&lines, r, c));
                c += 1;
            }
            if let Some(symbol) = symbol {
                let num = l[start..c]
                    .iter()
                    .fold(0, |n, c| n * 10 + (c - b'0') as usize);
                symbols.entry(symbol).or_insert(Vec::new()).push(num);
            }
            c += 1;
        }
    }
    let p1: usize = symbols.values().flatten().sum();
    let p2: usize = symbols
        .iter()
        .filter(|(&(_, _, s), v)| s == '*' && v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    println!("{} = {}", p1, p2);
    // (p1, p2)
}