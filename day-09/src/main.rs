use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let pt1: Vec<Vec<i32>> = binding
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
        .iter()
        .map(|row| predict_next_value(row.clone()))
        .collect::<Vec<Vec<i32>>>();

    println!("Rows: {:?}", pt1);

    println!(
        "Part 1: {}",
        pt1.iter()
            .map(|r| *r.last().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
    );

    let pt2 = pt1
        .iter()
        .map(|row| predict_previous_value(row.clone()))
        .collect::<Vec<Vec<i32>>>();

    println!(
        "Part 2: {}",
        pt2.iter()
            .inspect(|r| println!("{:?}", r))
            .map(|r| *r.first().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
    );
}

fn predict_previous_value(mut row: Vec<i32>) -> Vec<i32> {
    if row.iter().all(|i| *i == 0) {
        row = [vec![0 as i32], row].concat();
        return row;
    }

    let mut el_diffs: Vec<i32> = vec![];

    for (index, _) in row.iter().enumerate() {
        if index < row.len() - 1 {
            el_diffs.push(row[index + 1] - row[index]);
        }
    }

    let prev_val = row.first().unwrap() - *predict_previous_value(el_diffs).first().unwrap();
    row = [vec![prev_val], row].concat();
    return row;
}

fn predict_next_value(mut row: Vec<i32>) -> Vec<i32> {
    if row.iter().all(|i| *i == 0) {
        row.push(0);
        return row;
    }

    let mut el_diffs: Vec<i32> = vec![];

    for (index, _) in row.iter().enumerate() {
        if index < row.len() - 1 {
            el_diffs.push(row[index + 1] - row[index]);
        }
    }

    let next_val = row.last().unwrap() + *predict_next_value(el_diffs).last().unwrap();
    row.push(next_val);
    return row;
}
