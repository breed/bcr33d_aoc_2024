use bcr33d_aoc_2024::MyIn;
use std::collections::HashSet;

const MAX_DIM: usize = 71;

fn main() {
    let mut myin = MyIn::new();
    let mut grid = vec![vec!['.'; MAX_DIM]; MAX_DIM];
    let mut line_count = 0;
    let mut dfs_result = None;
    for _ in 0..3000 {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        line_count += 1;
        let prog = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        println!("{} {} dropping", prog[0], prog[1]);
        let x = prog[0];
        let y = prog[1];
        grid[prog[1]][prog[0]] = '#';
        if line_count == 3 {
            dfs_result = dfs(&mut grid, 0, 0);
        } else if line_count > 3 {
            if let Some(found) = &dfs_result {
                if !found.contains(&(x as i32, y as i32)) {
                    continue;
                }
            }
            for row in 0..MAX_DIM {
                for col in 0..MAX_DIM {
                    if grid[row][col] == 'O' {
                        grid[row][col] = '.';
                    }
                }
            }
            dfs_result = dfs(&mut grid, 0, 0);
            if dfs_result.is_none() {
                println!("break at {} {} in {} falls", x, y, line_count);
                break;
            }
        }
    }

    for row in 0..MAX_DIM {
        for col in 0..MAX_DIM {
            print!("{}", grid[row][col]);
        }
        println!();
    }
    println!("finished")
}

fn dfs(grid: &mut Vec<Vec<char>>, row: i32, col: i32) -> Option<HashSet<(i32, i32)>> {
    if row == (MAX_DIM - 1) as i32 && col == (MAX_DIM - 1) as i32 {
        let mut result = HashSet::new();
        result.insert((col as i32, row as i32));
        return Some(result);
    }
    if row >= 0
        && col >= 0
        && row < grid.len() as i32
        && col < grid[0].len() as i32
        && grid[row as usize][col as usize] == '.'
    {
        grid[row as usize][col as usize] = 'O';
        for m in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            if let Some(mut found) = dfs(grid, row + m.0, col + m.1) {
                found.insert((col as i32, row as i32));
                return Some(found);
            }
        }
    }
    None
}
