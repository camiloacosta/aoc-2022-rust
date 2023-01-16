mod utils;

const INPUT_FILE: &str = "./2022/inputs/input_2022_day_4.txt";

#[derive(Debug, Clone)]
struct Range {
    start: i32,
    end: i32
}

impl Range {
    fn contains(self: &Self, x: &Range) -> bool {
        self.start <= x.start && self.end >= x.end
    }

    fn touches(self: &Self, x: &Range) -> bool {
        self.contains(&Range { start: x.start, end: x.start }) || self.contains(&Range { start: x.end, end: x.end }) 
    }
}

fn parse_range(value: String) -> Vec<Range> {
    let ranges = value.split(",").map(|item| {
        let mut range_values = item.split("-");
        Range { 
            start: range_values.next().unwrap().parse::<i32>().unwrap(), 
            end: range_values.next().unwrap().parse::<i32>().unwrap()
        }
    });
    ranges.collect::<Vec<_>>()
}

fn process(comparison: &dyn Fn(Range, Range) -> bool) -> i32 {
    let mut count = 0;
    if let Ok(lines) = utils::strings::read_lines(INPUT_FILE) {
        for line in lines {
            if let Ok(value) = line {
                let ranges = parse_range(value);
                let a = ranges[0].clone();
                let b = ranges[1].clone();
                if comparison(a, b) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn process_part_1(a: Range, b: Range) -> bool {
    a.contains(&b) || b.contains(&a)
}

fn process_part_2(a: Range, b: Range) -> bool {
    a.touches(&b) || b.touches(&a)
}

fn main() {
    let p1 = process(&process_part_1);
    println!("part 1: {}", p1);

    let p2 = process(&process_part_2);
    println!("part 2: {}", p2);
}

