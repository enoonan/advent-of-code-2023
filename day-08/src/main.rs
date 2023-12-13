use num::integer::lcm;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
fn main() {
    let init_start = Instant::now();
    let binding = read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.split("\n").collect();

    let instructions: Vec<char> = lines.get(0).unwrap().chars().collect(); // next().unwrap().trim().chars().collect();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();

    let _ = lines[2..].into_iter().for_each(|l| {
        let (k, v) = l.split_once(" = ").unwrap();
        let val = v
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();

        network.insert(k, val);
    });
    let init_end = Instant::now();
    println!("Init took {} micros", (init_end - init_start).as_micros());

    let p1_steps = part1(&network, &instructions, "AAA");
    println!("Part 1: {}", p1_steps);

    part2(&network, &instructions);
}
fn part2(network: &HashMap<&str, (&str, &str)>, instructions: &Vec<char>) {
    let start_time = Instant::now();
    let root_steps = network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| *k)
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| part1(network, instructions, *n))
        .collect::<Vec<i32>>()
        .iter()
        .fold(1, |acc, steps_count| lcm(acc, *steps_count as i64));
    let end_time = Instant::now();
    println!("Pt 2 steps: {:?}", root_steps);
    println!("Pt 2 took: {} micros", (end_time - start_time).as_micros());
}

fn part1(
    network: &HashMap<&str, (&str, &str)>,
    instructions: &Vec<char>,
    start_label: &str,
) -> i32 {
    let mut node_label = start_label;
    let mut curr_node_val = network.get(node_label).unwrap();
    let mut steps = 0;
    let mut is_zzz = false;

    while !is_zzz {
        for direction in instructions {
            steps += 1;
            if direction == &'L' {
                node_label = curr_node_val.0
            } else {
                node_label = curr_node_val.1
            }

            if node_label.ends_with('Z') {
                is_zzz = true;
            }

            curr_node_val = network.get(node_label).unwrap();
        }
    }
    return steps;
}
