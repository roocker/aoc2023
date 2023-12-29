use std::fs;

#[derive(Debug)]
struct Boat {
    _id: usize,
    tmax: u64,
    dmax: u64,
    speed: u64,
    distance: u64,
}
impl Boat {
    fn new(_id: usize, max_time: u64, win_dist: u64, speed: u64, distance: u64) -> Self {
        Self {
            _id,
            tmax: max_time,
            dmax: win_dist,
            speed,
            distance,
        }
    }
}

fn race(r: &mut Boat) -> usize {
    let mut wincount = 0;
    for s in 0..r.tmax {
        r.speed = s;
        let ttravel = r.tmax - s;
        r.distance = s * ttravel;
        if r.distance > r.dmax {
            wincount += 1;
            // println!("winner: {:?}", r);
        }
    }
    wincount
}

fn day06a(input: &str) -> usize {
    let numbers = input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(':').unwrap();
            rest.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_times = &numbers[0];
    let win_dists = &numbers[1];

    let races = max_times
        .iter()
        .enumerate()
        .map(|(i, tmax)| Boat::new(i, *tmax, win_dists[i], 0, 0))
        .collect::<Vec<_>>();

    races
        .into_iter()
        .map(|mut boat| race(&mut boat))
        .product::<usize>()
}

fn day06b(input: &str) -> usize {
    let numbers = input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(':').unwrap();
            // println!("{:?}", rest);
            rest.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let tmax = &numbers[0];
    let dmax = &numbers[1];

    let mut r = Boat::new(0, *tmax, *dmax, 0, 0);

    race(&mut r)
}

fn main() {
    let filecontent = fs::read_to_string("input/day06.txt").unwrap();
    /* const filecontent: &str = "Time:      7  15   30
    Distance:  9  40  200"; */
    println!("result of day06a: {}", day06a(&filecontent));
    println!("---");
    println!("result of day06b: {}", day06b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_day06a() {
        assert_eq!(288, day06a(TEST_STRINGS))
    }
    #[test]
    fn test_day06b() {
        assert_eq!(71503, day06b(TEST_STRINGS))
    }
}
