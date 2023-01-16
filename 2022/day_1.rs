mod utils;

fn process_calories(items: usize) -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new();
    if let Ok(lines) = utils::strings::read_lines("./2022/inputs/input_2022_day_1.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    calories.push(count);
                    count = 0;
                    continue;
                }
                count += ip.parse::<i32>().unwrap();
            }
        }
    }

    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());

    calories[0..items].to_vec()
}

fn main() {
    let p1 = process_calories(1);
    println!("part 1: {}", p1[0]);

    let p2: i32 = process_calories(3).iter().sum();
    println!("part 2: {}", p2);
}

