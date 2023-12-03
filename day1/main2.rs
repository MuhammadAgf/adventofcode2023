use std::fs;
use std::collections::HashMap;

fn first_and_last(list: &[i64]) -> i64 {
    match list.len() {
        0 => 0,
        _ => &list[0] * 10 + &list[list.len() - 1],
    }
}

fn split_string(hm: &HashMap<&str, i64>, text: &str) -> Vec<i64> {
        
        let mut out: Vec<i64> = Vec::new();
        
        let mut start_idx = 0;

        while start_idx < text.len() {

            let mut match_find = false;
            
            for (k, v) in hm.into_iter() {
                if text[start_idx..].starts_with(k) {
                    match_find = true;
                    out.push(*v);
                    start_idx += k.len();
                    break;
                }
            }
            if !match_find {
                start_idx += 1
            }
        }
        out
}


fn main() {
    let content = fs::read_to_string("input.txt").expect("able to read file");
    let parts = content.split("\n");
    
    let hm = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1",1),
        ("2",2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0)
    ]);
    

    let number_list: Vec<Vec<i64>> = parts.map(|s| split_string(&hm, s)).collect();
    let result: i64 = number_list.iter().map(|list| first_and_last(list)).sum();
    println!("{}", result)
}
