use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

const INPUT_FILE: &str = "input.txt";

fn main() {
    let path = Path::new(INPUT_FILE);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    // Part 1
    let time_distances = parse_time_distances(&s);

    let d: Vec<u64> = time_distances
        .iter()
        .map(|td| return find_record_breaking_hold_times(td))
        .collect::<Vec<u64>>();

    let mult = d.iter().fold(1, |acc, &num| acc * num);
    println!("Part 1 answer: {}", mult);

    // Part 2 ... hardcode these, why not
    let start = Instant::now();
    let duration = 40709879;
    let record_to_beat = 215105121471005;
    let (min, max) = solve_for_x(duration, record_to_beat);
    let end = Instant::now();
    println!("Part 2 answer: {}", max - min + 1); // off by one error

    println!("Part 2 took {} microseconds", (end - start).as_micros())
}

//  used for testing part two
// fn get_distance_for_hold_time(hold_time: &i64, duration: &u64) -> i128 {
//     return (*duration as i128 - *hold_time as i128) * *hold_time as i128;
// }

// used for testing part two
// fn hold_time_breaks_record(hold_time: &i64, duration: &u64, record: &u64) -> bool {
//     return get_distance_for_hold_time(hold_time, duration) > *record as i128;
// }

fn parse_time_distances(s: &str) -> Vec<(u64, u64)> {
    if let [times_str, distances_str, ..] = s.split("\n").into_iter().collect::<Vec<&str>>()[..2] {
        let times = &times_str[10..]
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .unwrap();
        let distances = &distances_str[10..]
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .unwrap();

        let mut time_distances: Vec<(u64, u64)> = [].to_vec();
        for (i, t) in times.iter().enumerate() {
            time_distances.push((*t, distances[i]));
        }

        return time_distances;
    } else {
        panic!("could not parse strings")
    }
}

fn solve_for_x(duration: u64, record_to_beat: u64) -> (i64, i64) {
    let x_min: i64;
    let x_max: i64;
    let f_duration = duration as f64;
    let f_record_d = record_to_beat as f64;

    let discriminant: f64 = f_duration.powi(2) - (4.0 * f_record_d);

    let f_x_max: f64 = (-f_duration - discriminant.sqrt()) / -2.0;
    let f_x_min: f64 = (-f_duration + discriminant.sqrt()) / -2.0;

    x_min = f_x_min.ceil() as i64 + 1; // min needs to be just above the record
    x_max = f_x_max.floor() as i64 - 1; // max needs to be just below the record

    return (x_min, x_max);
}

fn find_record_breaking_hold_times((duration, record_distance): &(u64, u64)) -> u64 {
    let mut result = 0;
    let mut hit_min = false;

    for hold_time in 1..*duration {
        let d = (*duration - hold_time) * hold_time;
        if d > *record_distance {
            hit_min = true;
            result += 1
        } else {
            if hit_min {
                break; // we have passed the realm of record breaking hold times
            }
        }
    }

    result
}
