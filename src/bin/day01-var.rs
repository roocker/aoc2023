#[inline(always)]
fn day01b_short(line: &[u8], i: usize) -> Option<usize> {
    const NUMBERS: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(NUMBERS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}

fn main() {
    // const TEST_STRINGS: &str = "aeightseven651\nseven90nine0\n";
    // println!("result of day01b: {}", day01b(TEST_STRINGS));
    let filecontent = fs::read_to_string("input.txt").unwrap();
    println!("result of day01b: {}", day01b(&filecontent));
    println!("---- NOW SHORTER, FASTER STRONGER:");
    // println!("result of day01b: {}", day01b(&filecontent));
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                (0..line.len()).find_map(|i| day01b_short(line, i)).unwrap() * 10
                    + (0..line.len())
                        .rev()
                        .find_map(|i| day01b_short(line, i))
                        .unwrap()
            })
            .sum::<usize>()
    );
}
