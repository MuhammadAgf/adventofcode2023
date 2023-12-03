use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt").expect("able to read file");
    let parts = content.split("\n");
    let result: i32 = parts.map( |s| {
        match s {
            "" => { 0 }
            _ => {
                let (game, values) = s.split_once(": ").expect("split by : ");
                let (_, game_num) = game.split_once(" ").expect("split by space to get game_num");
                let value_parts = values.split("; ");
                
                let game_result = value_parts.map(
                    |value_part| {
                        let candies = value_part.split(", ");
                        candies.map(|candy| {
                            let (candy_count_str, candy_name) = candy.split_once(" ").expect("split by space");
                            let candy_count = candy_count_str.parse::<i32>().unwrap();
                            match candy_name {
                                "red" => candy_count <= 12,
                                "green" => candy_count <= 13,
                                _ => candy_count <= 14,
                            }
                        }).all(|val| val)
                    }
                ).all(|val| val);

                println!("wth {:?} {}", s, game_result);
                if game_result {
                    game_num.parse::<i32>().unwrap()
                } else {
                    0
                }
            }
        }
    }).sum();
    println!("{}", result)
}
