use std::env;
use std::fs;


fn solve_one_iter(grid: &Vec<Vec<char>>) -> (i64, Vec<Vec<char>>) {
    // First pass: find all cells to remove (without modifying)
    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                // count adjacent '@' in 8-neighborhood
                let mut count = 0;
                for di in -1isize..=1 {
                    for dj in -1isize..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && nj >= 0 {
                            let ni = ni as usize;
                            let nj = nj as usize;
                            if ni < grid.len() && nj < grid[ni].len() {
                                if grid[ni][nj] == '@' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
                if count < 4 {
                    to_remove.push((i, j));
                }
            }
        }
    }

    // Second pass: create new grid with removals
    let mut new_grid = grid.clone();
    for (i, j) in &to_remove {
        new_grid[*i][*j] = '.';
    }

    (to_remove.len() as i64, new_grid)
}
fn solve(lines: Vec<String>) -> (i64, i64) {
    let initial_grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let (part1, _) = solve_one_iter(&initial_grid);

    let mut part2 = 0;
    let mut current_grid = initial_grid;

    loop {
        let (removed, new_grid) = solve_one_iter(&current_grid);

        if removed == 0 {
            break;
        }

        part2 += removed;
        current_grid = new_grid;
    }
    (part1, part2)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    // parse input into list of list of strings
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let (part1, part2) = solve(lines);
    println!("{}", part1);
    println!("{}", part2);
}
