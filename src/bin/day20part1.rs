use bcr33d_aoc_2024::MyIn;
use std::collections::HashMap;

const WALL_NUM: i32 = 99999;

fn main() {
    println!("day20 part 1");
    let mut myin = MyIn::new();
    let mut grid = Vec::new();

    // find the start and end
    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = 0;
    let mut end_col = 0;

    {
        let mut row = 0 as usize;
        loop {
            let line = myin.read_line();
            if line.is_empty() {
                break;
            }
            grid.push(
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        'S' => {
                            start_row = row;
                            start_col = col;
                            0
                        }
                        'E' => {
                            end_row = row;
                            end_col = col;
                            -1
                        }
                        '.' => -1,
                        _ => 99999,
                    })
                    .collect::<Vec<i32>>(),
            );
            row += 1;
        }

    }

    {
        let mut row = start_row;
        let mut col = start_col;

        let mut move_count = 1;

        while row != end_row || col != end_col {
            for m in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                // we have a border at 0 so we don't have to worry about boundaries
                let new_row = (row as i32 + m.0) as usize;
                let new_col = (col as i32 + m.1) as usize;
                if grid[new_row as usize][new_col as usize] == -1 {
                    grid[new_row as usize][new_col as usize] = move_count;
                    move_count += 1;
                    row = new_row;
                    col = new_col;
                    break;
                }
            }
        }
        print_grid(&grid);


    }

    let mut save100ps = 0;
    // find short cuts
    let mut cuts = HashMap::new();
    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {
            if grid[row][col] != WALL_NUM {
                // we only care about walls
                continue;
            }
            if row == 2 && col == 1 {
                println!("here");
            }
            let vdiff = if grid[row - 1][col] != WALL_NUM && grid[row + 1][col] != WALL_NUM {
                (grid[row - 1][col] - grid[row + 1][col]).abs()-2
            } else {
                0
            };
            let hdiff = if grid[row][col - 1] != WALL_NUM && grid[row][col + 1] != WALL_NUM {
                (grid[row][col - 1] - grid[row][col + 1]).abs()-2
            } else {
                0
            };
            if hdiff > 1 {
                *cuts.entry(hdiff).or_insert(0) += 1;
            }
            if vdiff > 1 {
                *cuts.entry(vdiff).or_insert(0) += 1;
            }
            if hdiff >= 100 {
                save100ps += 1;
            }
            if vdiff >= 100 {
                save100ps += 1;
            }
        }
    }

    print_grid(&grid);

    let mut sorted_cuts = cuts.iter().collect::<Vec<(&i32, &i32)>>();
    sorted_cuts.sort_by_key(|&(k,_)| k);
    for (k, v) in sorted_cuts {
        println!("{} {}", k, v);
    }
    println!("save 100ps {}", save100ps);
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for c in row {
            print!("{:6}", c);
        }
        println!();
    }
    println!();
}
