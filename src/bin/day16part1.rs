use std::collections::VecDeque;
use bcr33d_aoc_2024::MyIn;
use crate::Rotate::{Left, Right, Straight};

struct Position {
    row: i32,
    col: i32,
    dir: (i32, i32),
    prev: (i32, i32),
    cost: u64,
}

enum Rotate {
    Straight,
    Left,
    Right,
}
fn main() {
    println!("day16 part 1");
    let mut myin = MyIn::new();
    let mut grid = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(
            line.chars().collect::<Vec<char>>(),
        );
    }

    // find the robot
    let mut start_row = 0;
    let mut start_col = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                start_row = row;
                start_col = col;
                println!("found S at {} {}", row, col);
                break;
            }
        }
    }

    let mut costs = vec![vec![u64::MAX; grid[0].len()]; grid.len()];
    let mut prevs = vec![vec![Vec::new(); grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut end_row = 0;
    let mut end_col = 0;
    queue.push_back(Position{row: start_row as i32, col: start_col as i32, dir: (0, 1), prev: (0,0), cost: 0});
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let row = pos.row as usize;
        let col = pos.col as usize;
        match grid[row][col] {
            'S' | '.' => {
                if pos.cost == costs[row][col] {
                    println!("found equal at {} {}, pushing {} {}", row, col, pos.prev.0, pos.prev.1);
                    prevs[row][col].push(pos.prev);
                } else if pos.cost < costs[row][col] {
                    costs[row][col] = pos.cost;
                    prevs[row][col] = vec![pos.prev];
                    println!("found better at {} {}, initializing {} {}", row, col, pos.prev.0, pos.prev.1);
                    queue.push_back(pos_rotate(&pos, Straight));
                    queue.push_back(pos_rotate(&pos, Left));
                    queue.push_back(pos_rotate(&pos, Right));
                }
            },
            '#' => continue,
            'E' => {
                if pos.cost == costs[row][col] {
                    println!("found equal at {} {}, pushing {} {}", row, col, pos.prev.0, pos.prev.1);
                    prevs[row][col].push(pos.prev);
                } else if pos.cost < costs[row][col] {
                    println!("found better at {} {}, initializing {} {}", row, col, pos.prev.0, pos.prev.1);
                    costs[row][col] = pos.cost;
                    prevs[row][col] = vec![pos.prev];
                    end_row = row;
                    end_col = col;
                }
            },
            _ => {
                panic!("bad char {}", grid[row][col]);
            }
        }
      //  print_costs(&costs);
    }

    // fill in the prevs
    let mut queue = VecDeque::new();
    queue.push_back((end_row as i32, end_col as i32));
    let mut total = 1;
    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        if grid[row as usize][col as usize] != 'O' {
            total += 1;
            grid[row as usize][col as usize] = 'O';
            for (row, col) in &prevs[row as usize][col as usize] {
                println!("pushing prev {} {}", row, col);
                queue.push_back((*row, *col));
            }
        }
    }
    print_grid(&grid);
    println!("cost {}", costs[end_row][end_col]);
    println!("chairs {}", total);
}

fn pos_rotate(pos: &Position, rot: Rotate) -> Position {
    let new_dir = match rot {
        Straight => pos.dir,
        Left => (-pos.dir.1, pos.dir.0),
        Right => (pos.dir.1, -pos.dir.0),
    };
    let cost_inc = if let Straight = rot { 1 } else { 1001 };
    Position{row: pos.row + new_dir.0, col: pos.col + new_dir.1, dir: new_dir, prev: (pos.row, pos.col), cost: pos.cost + cost_inc }
}

fn print_costs(costs: &Vec<Vec<u64>>) {
    println!("-------------");
    for row in costs {
        for c in row {
            if *c == u64::MAX {
                print!("{:<6}", "MAX");
            } else {
                print!("{:<6}", c);
            }
        }
        println!();
    }
    println!();
}
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
