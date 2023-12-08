use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let card_types = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
    ];

    let score: i32 = read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            (
                hand_type(
                    &hand
                        .trim()
                        .split("")
                        .filter(|c| c.len() > 0)
                        .counts()
                        .values()
                        .sorted_by(|a, b| b.cmp(a)) // reverse sort
                        .collect::<Vec<&usize>>(),
                ),
                hand.chars().into_iter().map(|card| {
                    // transform cards to score values
                    card_types
                        .iter()
                        .position(|c| **c == card.to_string())
                        .unwrap()
                }),
                bid.parse::<i32>().unwrap(),
            )
        })
        .sorted_by(|a, b| {
            if a.0 != b.0 {
                return a.0.cmp(&b.0);
            }
            for (index, card) in a.1.clone().into_iter().enumerate() {
                let b1_val = b.1.clone().nth(index).unwrap();
                if card != b1_val {
                    return card.cmp(&b1_val);
                }
            }
            return Ordering::Equal;
        })
        .enumerate()
        .map(|(index, hand)| (index as i32 + 1) * hand.2)
        .sum();

    println!("Part 1: {}", score);
}

fn hand_type(hand_vals: &Vec<&usize>) -> i32 {
    if hand_vals.len() == 1 {
        return 6; // if there's no second value then it's 5 of a kind :D
    }

    return match (hand_vals[0], hand_vals[1]) {
        (4, 1) => 5, // 4 of a kind
        (3, 2) => 4, // Danny Tanner
        (3, 1) => 3, // 3 of a kind
        (2, 2) => 2, // 2 pair
        (2, 1) => 1, // 1 pair
        (1, _) => 0, // high card
        (_, _) => 0, // shouldn't ever get here
    };
}
