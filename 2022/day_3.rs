mod utils;

fn process_rugstack(chunk_size: usize, get_repeated_char: &dyn Fn(&[String]) -> char) -> i32 {
    let mut priority: i32 = 0;
    if let Ok(lines) = utils::strings::read_lines("./2022/inputs/input_2022_day_3.txt") {
        let collected: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
        let chunks = collected.chunks(chunk_size);
        for chunk in chunks {
            let char_code: i32 = get_repeated_char(chunk) as i32;
            let mut value: i32 = 26 + char_code - 64;
            if char_code > 96 { value = char_code - 96; }
            priority += value;
        }
    }

    priority
}

fn process_part_1(chunk: &[String]) -> char {
    let value = chunk[0].clone();
    let first = value[..(value.len() / 2)].to_string();
    let second = value[(value.len() / 2)..].to_string();
    let mut repeated_char: char = ' ';
    for c in second.chars() {
        if first.contains(c) {
            repeated_char = c;
            break
        }
    }
    if repeated_char == ' ' { panic!("cannot find repeated char"); }
    repeated_char
}

fn process_part_2(chunk: &[String]) -> char {
    let mut repeated_char = ' ';
    let (first, second, third) = (chunk[0].clone(), chunk[1].clone(), chunk[2].clone());
    for c in third.chars() {
        if first.contains(c) && second.contains(c) {
            repeated_char = c;
            break
        }
    }
    if repeated_char == ' ' { panic!("cannot find repeated char"); }
    return repeated_char
}

fn main() {
    let p1 = process_rugstack(1, &process_part_1);
    println!("part 1: {}", p1);

    let p2 = process_rugstack(3, &process_part_2);
    println!("part 2: {}", p2);
}

