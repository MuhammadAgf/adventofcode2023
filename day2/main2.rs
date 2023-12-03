use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt").expect("able to read file");
    let parts = content.split("\n");
    let result: i32 = parts.map( |s| {
        match s {
            "" => { 0 }
            _ => {
                let (_, values) = s.split_once(": ").expect("split by : ");
                let value_parts = values.split("; ");
                
                let game_result = value_parts.map(
                    |value_part| {
                        let candies = value_part.split(", ");
                        candies.map(|candy| {
                            let (candy_count_str, candy_name) = candy.split_once(" ").expect("split by space");
                            let candy_count = candy_count_str.parse::<i32>().unwrap();
                            (candy_name, candy_count)
                        })
                    }
                ).flatten();

                let red = game_result.clone().filter(|&(candy_name, _)| candy_name == "red").map(|(_, candy_result)| candy_result).max().unwrap_or(0);
                let blue = game_result.clone().filter(|&(candy_name, _)| candy_name == "blue").map(|(_, candy_result)| candy_result).max().unwrap_or(0);
                let green = game_result.filter(|&(candy_name, _)| candy_name == "green").map(|(_, candy_result)| candy_result).max().unwrap_or(0);

                red * blue * green
            }
        }
    }).sum();
    println!("{}", result)
}
