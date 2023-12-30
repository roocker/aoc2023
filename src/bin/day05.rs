use std::sync::{Arc, Mutex};
use std::{fs, thread, time::Duration};

fn seedtolocation(seed: u64, seed_to_location_map: &[Vec<Vec<u64>>]) -> u64 {
    let mut dest = seed;

    for (i, map) in seed_to_location_map.iter().enumerate() {
        for v in map {
            if (v[1]..v[1] + v[2]).contains(&dest) {
                dest = v[0] + (dest - v[1]);
                break;
            } else {
                dest;
            }
        }
        // println!("seed: {} -> map#{} : {:?}: -> dest: {}", seed, i, map, dest);
    }
    dest
}
fn get_seed_data(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let mut seeds = Vec::new();
    let mut seed_to_location_map: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut heading: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        if line.starts_with("seeds:") {
            let numbers: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            seeds.push(numbers);
            continue;
        }
        if line.ends_with(':') {
            if !heading.is_empty() {
                seed_to_location_map.push(heading);
                heading = Vec::new();
            }
        } else if !line.is_empty() {
            let numbers: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            heading.push(numbers);
        }
    }
    if !heading.is_empty() {
        seed_to_location_map.push(heading);
    }

    // println!("Seeds {:?}", seeds);
    // println!("seed to location map:{:?}", seed_to_location_map);
    (
        seeds.into_iter().flatten().collect::<Vec<u64>>(),
        seed_to_location_map,
    )
}

fn day05a(input: &str) -> u64 {
    println!("PART A:\n");
    let (seeds, seed_to_location_map) = get_seed_data(input);

    println!("Seeds to check: {:?}", seeds);
    let min_location = seeds
        .iter()
        .map(|seed| {
            let location = seedtolocation(*seed, &seed_to_location_map);
            // println!("seed {} -> location: !!!!{}!!!!", seed, location);
            location
        })
        .min()
        .unwrap();

    min_location
}

/* fn day05b(input: &str) -> u64 {
    println!("PART B:\n");
    let (seedranges, seed_to_location_map) = get_seed_data(input);

    // let mut (start, lengh) : Vec<(u64,u64)> = Vec::new();

    let mut seeds = Vec::new();
    let mut lengths = Vec::new();
    let mut starts = Vec::new();

    for (i, num) in seedranges.iter().enumerate() {
        if i % 2 == 0 {
            starts.push(*num);
        } else {
            lengths.push(*num);
        }
    }

    // println!("start {:?} range  {:?}", starts, lengths);

    for (&start, &length) in starts.iter().zip(lengths.iter()) {
        println!("{} .. {}: ", start, length,);
        seeds.extend(start..start + length);
    }

    println!("Seeds to check {:?}", seeds);

    let min_location = seeds
        .iter()
        .map(|seed| seedtolocation(*seed, &seed_to_location_map))
        .min()
        .unwrap();


    min_location
} */

/* let thread = thread::spawn(|| {
    for i in 1..10 {
        println!("hi {}", i);
        // thread::sleep(Duration::from_millis(10));
    }
});
thread.join().unwrap(); */

fn day05b(input: &str) -> u64 {
    println!("PART B:\n");
    let (seedranges, seed_to_location_map) = get_seed_data(input);

    // Create a vector to hold the child threads
    let mut children = vec![];

    // Wrap the seed_to_location_map in an Arc and Mutex for thread safety
    let map = Arc::new(Mutex::new(seed_to_location_map));

    for i in 0..seedranges.len() / 2 {
        // Clone the Arc to be used in this thread
        let map = Arc::clone(&map);

        let seedranges = seedranges.clone(); // Add this line
                                             // Spawn a new thread
        let child = thread::spawn(move || {
            // Lock the map for this thread to use
            println!(
                "spawned new thread creating seeds: {:?},",
                seedranges[2 * i],
            );
            let map = map.lock().unwrap();
            let start = seedranges[2 * i];
            let length = seedranges[2 * i + 1];
            let mut seeds = Vec::new();
            seeds.extend(start..start + length);
            seeds
        });

        // Collect the threads
        children.push(child);
    }

    // Collect the results from the threads
    let seeds: Vec<u64> = children
        .into_iter()
        .map(|child| child.join().unwrap())
        .flatten()
        .collect();

    // println!("Seeds to check {:?}", seeds);

    // Create a vector to hold the child threads
    let mut children = vec![];

    for seed in seeds {
        // Clone the Arc to be used in this thread
        let map = Arc::clone(&map);

        // Spawn a new thread
        let child = thread::spawn(move || {
            // Lock the map for this thread to use
            let map = map.lock().unwrap();
            seedtolocation(seed, &map)
        });

        // Collect the threads
        children.push(child);
    }

    // Collect the results from the threads
    let locations: Vec<u64> = children
        .into_iter()
        .map(|child| child.join().unwrap())
        .collect();

    // Find the minimum location
    let min_location = locations.iter().min().unwrap();

    *min_location
}

fn main() {
    let filecontent = fs::read_to_string("input/day05.txt").unwrap();

    /* const filecontent: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4"; */

    println!("result of day05a: {}", day05a(&filecontent));
    //result of day05a: 226172555

    println!("---");
    println!("result of day05b: {}", day05b(&filecontent));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRINGS: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_day05a() {
        assert_eq!(35, day05a(TEST_STRINGS))
    }
    #[test]
    fn test_day05b() {
        assert_eq!(46, day05b(TEST_STRINGS))
    }
}
