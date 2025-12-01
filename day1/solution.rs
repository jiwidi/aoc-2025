use std::env;
use std::fs;

fn solve(lines: &[String]) -> (i64, i64) {
    let mut position: i64 = 50;
    let mut zero_counter: i64 = 0;
    let mut pass_through_zero: i64 = 0;

    for line in lines {
        let distance: i64 = line[1..].parse().unwrap();
        let old_position = position;

        if line.starts_with('L') {
            position -= distance;
        } else {
            position += distance;
        }

        // Count times we pass through 0 during rotation (excluding final position)
        pass_through_zero += if position > old_position {
            (position - 1).div_euclid(100) - old_position.div_euclid(100)
        } else if position < old_position {
            (old_position - 1).div_euclid(100) - position.div_euclid(100)
        } else {
            0
        };

        position = position.rem_euclid(100);
        if position == 0 {
            zero_counter += 1;
        }
    }

    (zero_counter, zero_counter + pass_through_zero)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let (part1, part2) = solve(&lines);
    println!("{}", part1);
    println!("{}", part2);
}
