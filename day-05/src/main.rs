use rayon::prelude::*;
use std::fs::File;
use std::i64;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn parse(s: String) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let mut maps: Vec<Vec<Vec<i64>>> = [].to_vec();
    let mut map_index = 0;
    let mut seeds: Vec<i64> = [].to_vec();

    for line in s.split("\n").into_iter() {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains("seeds:") {
            let seed_nums = line[7..]
                .split_whitespace()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()
                .unwrap_or_else(|e| {
                    panic!("Failed to parse integer: {}", e);
                });

            for seed in seed_nums {
                seeds.append(&mut [seed].to_vec())
            }
        } else if line.contains("map:") {
            maps.push([].to_vec());
            map_index += 1;
        } else {
            // print!("{}\n", line);
            let items = line
                .split_whitespace()
                .map(|num| num.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()
                .unwrap_or_else(|e| {
                    panic!("Failed to parse integer: {}", e);
                });
            maps[map_index - 1].push(items);
        }
    }

    return (seeds, maps);
}

fn main() {
    let start = Instant::now();
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("read string"),
    }

    let (seeds, maps) = parse(s);

    let pt_1_result = get_locations(&seeds, &maps);
    println!("Part 1 result: {}", pt_1_result.iter().min().unwrap());
    let end = Instant::now();
    println!("Part 1 took: {} milliseconds", (end - start).as_millis())
    // let start_get_seeds = Instant::now();
    // let pt_2_seeds = get_pt_2_seeds(seeds);
    // let end_get_seeds = Instant::now();
    // println!(
    //     "Getting seeds took {} seconds",
    //     (end_get_seeds - start_get_seeds).as_secs()
    // );
    // println!("Number of seeds: {}", pt_2_seeds.len()); // 2567694082

    // let pt_2_locations = get_locations(&pt_2_seeds, &maps);

    // println! {"Part 2 Min Location: {}", pt_2_locations.iter().min().unwrap()}
}

fn get_locations(seeds: &Vec<i64>, maps: &Vec<Vec<Vec<i64>>>) -> Vec<i64> {
    return seeds
        .par_iter()
        .map(|seed| get_seed_location(seed, maps))
        .collect();
}

fn get_seed_location(seed: &i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    return maps.iter().fold(*seed, |mut acc, soil_map| {
        let soil_num = soil_map.iter().find_map(|soil_line| {
            if let [dst, src, r_size] = soil_line.as_slice() {
                let range_end = src + r_size;
                if acc >= *src && acc < range_end {
                    return Some(acc + (*dst - *src));
                }

                return None;
            } else {
                panic!("soil_line does not have exactly three elements");
            }
        });

        if let Some(soil_num) = soil_num {
            acc = soil_num;
        }

        return acc;
    });
}

fn get_pt_2_seeds(seeds: Vec<i64>) -> Vec<i64> {
    let mut results: Vec<i64> = Vec::new();

    for seed_range in seeds.chunks(2) {
        if seed_range.len() == 2 {
            let range_end = seed_range[0] + seed_range[1];
            results.extend(seed_range[0]..range_end);
        }
    }

    results
}
