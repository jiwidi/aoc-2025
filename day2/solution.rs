use std::env;
use std::fs;

fn is_invalid_part1(num: i64) -> bool {
    let s = num.to_string();
    let len = s.len();
    // Must have even length to be "sequence repeated twice"
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    // First half must equal second half
    &s[..half] == &s[half..]
}

fn is_invalid_part2(num: i64) -> bool {
    let s = num.to_string();
    let len = s.len();
    // Try each possible substring length (1 to len/2)
    // The substring must repeat at least twice, so max length is len/2
    for sub_len in 1..=len / 2 {
        if len % sub_len == 0 {
            let pattern = &s[..sub_len];
            let repeats = len / sub_len;
            // Check if pattern repeated 'repeats' times equals the string
            if pattern.repeat(repeats) == s {
                return true;
            }
        }
    }
    false
}

fn solve(lines: &[String]) -> (i64, i64) {
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for line in lines {
        let (left, right) = line.split_once('-').unwrap();
        let a: i64 = left.parse().unwrap();
        let b: i64 = right.parse().unwrap();

        for num in a..=b {
            if is_invalid_part1(num) {
                part1_sum += num;
            }
            if is_invalid_part2(num) {
                part2_sum += num;
            }
        }
    }
    (part1_sum, part2_sum)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    // Parse str of 77-88,22-34,5-10 into vec of strings "77-88", "22-34", "5-10"
    let lines: Vec<String> = input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let (part1, part2) = solve(&lines);
    println!("{}", part1);
    println!("{}", part2);
}
