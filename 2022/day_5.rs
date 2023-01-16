mod utils;

const INPUT_FILE: &str = "./2022/inputs/input_2022_day_5.txt";
const CRATE_STR_LEN: usize = 4;

struct Command {
    from: usize,
    to: usize,
    amount: usize
}

fn parse_commands(input: String) -> Command {
    let mut slices = input.split(" ");
    let amount = slices.nth(1).unwrap().parse::<usize>().unwrap();
    let from = slices.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    let to = slices.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    Command { from, to, amount }
}

fn process(move_crates: &dyn Fn(&mut Vec<char>, usize) -> Vec<char>) -> String {
    let mut top_stacks: String = "".to_owned();
    if let Ok(lines) = utils::strings::read_lines(INPUT_FILE) {
        let mut pickup_crates = true;

        let mut crate_lines: Vec<String> = vec![];
        let mut commands: Vec<Command> = vec![];
        for line in lines {
            if let Ok(row) = line {
                if row == "" { 
                    pickup_crates = false;
                    continue
                }

                if pickup_crates { crate_lines.insert(0, row.clone()); }
                else { commands.push(parse_commands(row)) }
            }
        }
        let base = crate_lines.remove(0);
        let stack_count = (base.len() + 1) / CRATE_STR_LEN;
        let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];

        for crate_line in crate_lines {
            for i in 0..stack_count {
                let mut chars = crate_line.chars();
                let crate_id = chars.nth((i * CRATE_STR_LEN) + 1).unwrap();
                if crate_id == ' ' { continue }
                stacks[i].push(crate_id);
            }
        }

        for command in commands {
            let load: Vec<char> = move_crates(&mut stacks[command.from], command.amount);
            stacks[command.to].extend(load);
        }

        for mut stack in stacks {
            top_stacks += &stack.pop().unwrap().clone().to_string();
        }
    }

    top_stacks
}

fn process_part_1(original_stack: &mut Vec<char>, amount: usize) -> Vec<char> {
    let mut load: Vec<char> = vec![];
    for _ in 0..amount {
        let current_crate = original_stack.pop().unwrap().clone();
        load.push(current_crate);
    }

    load
}

fn process_part_2(original_stack: &mut Vec<char>, amount: usize) -> Vec<char> {
    let mut load: Vec<char> = vec![];
    for _ in 0..amount {
        let current_crate = original_stack.pop().unwrap().clone();
        load.insert(0, current_crate);
    }

    load
}

fn main() {
    let p1 = process(&process_part_1);
    println!("part 1: {}", p1);

    let p2 = process(&process_part_2);
    println!("part 2: {}", p2);
}

