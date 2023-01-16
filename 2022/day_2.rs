mod utils;

#[derive(PartialEq, Debug, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors
}

fn map_oponent_play(input: &str) -> Play {
    match input {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("input nor supoprted")
    }
}

fn calculate_points(theirs: Play, mine: Play) -> i32 {
    let mut points: i32 = match mine {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3
    };

    if theirs == mine { points += 3 }
    else {
        points += match (theirs, mine) {
            (Play::Rock, Play::Scissors) => 0,
            (Play::Paper, Play::Rock) => 0,
            (Play::Scissors, Play::Paper) => 0,
            _ => 6,
        }
    }

    points
}

fn process_game(play_calculation: &dyn Fn(&Play, &str) -> Play) -> i32 {
    let mut points = 0;
    if let Ok(lines) = utils::strings::read_lines("./2022/inputs/input_2022_day_2.txt") {
        for line in lines {
            if let Ok(value) = line {
                let mut plays = value.split(' ');
                let (theirs, mine) = (plays.next().unwrap(), plays.next().unwrap());
                let oponent_play = map_oponent_play(theirs);
                let my_play = play_calculation(&oponent_play, mine);
                points += calculate_points(oponent_play, my_play);
            }
        }
    }

    points
}

fn map_input_to_play_part_1(_: &Play, input: &str) -> Play {
    match input {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("input nor supoprted")
    }
}

fn map_input_to_play_part_2(oponent: &Play, input: &str) -> Play {
    // Y: to draw
    if input == "Y" { return oponent.clone(); }
    match (&oponent, input) {
        // X: to lose
        (Play::Rock, "X") => Play::Scissors,
        (Play::Paper, "X") => Play::Rock,
        (Play::Scissors, "X") => Play::Paper,
        // Z: to win
        (Play::Rock, "Z") => Play::Paper,
        (Play::Paper, "Z") => Play::Scissors,
        (Play::Scissors, "Z") => Play::Rock,
        _ => panic!("input not supported")
    }
}

fn main() {
    let p1 = process_game(&map_input_to_play_part_1);
    println!("part 1: {}", p1);

    let p2 = process_game(&map_input_to_play_part_2);
    println!("part 2: {}", p2);
}

