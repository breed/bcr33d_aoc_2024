use bcr33d_aoc_2024::MyIn;
use std::collections::HashMap;

const WALL_NUM: i32 = 99999;

fn main() {
    println!("day20 part 2");
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

    let mut save100ps = 0 as u64;
    // find short cuts
    let mut cuts = HashMap::new();
    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {
            let end_s = grid[row][col];
            if end_s == WALL_NUM {
                // we only care about paths
                continue;
            }
            for drow in -20..21 {
                let new_row = row as i32 + drow;
                if new_row < 0 || new_row >= grid.len() as i32 {
                    continue;
                }
                let dcol_size = 20 - drow.abs();
                for dcol in -dcol_size..dcol_size+1 {
                    let new_col = col as i32 + dcol;
                    if new_col < 0 || new_col >= grid[0].len() as i32 {
                        continue;
                    }
                    let s = grid[new_row as usize][new_col as usize];
                    if s < end_s {
                        let diff = end_s - s - drow.abs() - dcol.abs();
                        if diff % 2 != 0 {
                            println!("bad diff {},{} has {} {},{} has {} {}", row, col, end_s, new_row, new_col, s, diff);
                        }
                        if diff > 1 {
                            *cuts.entry(diff).or_insert(0) += 1;
                            if diff >= 100 {
                                save100ps += 1;
                            }
                        }
                    }
                }
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
