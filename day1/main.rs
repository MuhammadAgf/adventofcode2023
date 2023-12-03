use std::fs;

fn first_and_last(list: &[i64]) -> i64 {
    match list.len() {
        0 => 0,
        _ => &list[0] * 10 + &list[list.len() - 1],
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("able to read file");
    let parts = content.split("\n");
    let number_list: Vec<Vec<i64>> = parts
        .map(|s| { s.chars().filter_map(|num| num.to_string().parse::<i64>().ok()).collect() }).collect();
    let result: i64 = number_list.iter().map(|list| first_and_last(list)).sum();
    println!("{}", result)
}
