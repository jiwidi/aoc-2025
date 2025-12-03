use std::env;
use std::fs;

fn pick_recursive(bank: &[i64], start: usize, k: usize) -> Vec<i64> {
    if k == 0 {
        return Vec::new();
    }

    let n = bank.len();

    // The next digit must come from [start ..= n-k]
    let end = n - k;
    let mut best_digit = -1;
    let mut best_index = start;

    for i in start..=end {
        if bank[i] > best_digit {
            best_digit = bank[i];
            best_index = i;
        }
    }

    let mut result = Vec::with_capacity(k);
    result.push(best_digit);

    let mut tail = pick_recursive(bank, best_index + 1, k - 1);
    result.append(&mut tail);

    result
}

fn solve(lines: &[Vec<i64>]) -> (i64, i64) {
    let mut solution_one: i64 = 0;
    let mut solution_two: i64 = 0;

    for bank in lines {
        if bank.len() >= 2 {
            let top2 = pick_recursive(bank, 0, 2);
            let val = top2[0] * 10 + top2[1];
            solution_one += val;
        }

        if bank.len() >= 12 {
            let top12 = pick_recursive(bank, 0, 12);
            let mut number: i64 = 0;
            for d in top12 {
                number = number * 10 + d;
            }
            solution_two += number;
        }
    }

    (solution_one, solution_two)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");

    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Non-digit in input") as i64)
                .collect()
        })
        .collect();

    let (part1, part2) = solve(&lines);
    println!("{}", part1);
    println!("{}", part2);
}
