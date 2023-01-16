use std::collections::HashSet;
use std::iter::FromIterator;

mod utils;

const INPUT_FILE: &str = "./2022/inputs/input_2022_day_6.txt";

fn process(win_size: usize) -> usize {
    let mut packet_marker = 0;
    if let Ok(mut lines) = utils::strings::read_lines(INPUT_FILE) {
        let line = lines.next().unwrap().expect("read line");
        let chars: Vec<char> = line.chars().collect();
        let char_windows = chars.windows(win_size);
        for (i, window) in char_windows.into_iter().enumerate() {
            let c: HashSet<&char> = HashSet::from_iter(window);
            if c.len() == win_size {
                packet_marker = i + win_size;
                break
            }
        }
    }

    packet_marker
}

fn main() {
    let p1 = process(4);
    println!("part 1: {}", p1);

    let p2 = process(14);
    println!("part 2: {}", p2);
}

