use std::cmp::min;
use std::collections::{HashMap, HashSet};
use bcr33d_aoc_2024::MyIn;
use Heading::{Straight, Left, Right};

struct Position {
    row: i32,
    col: i32,
    dir: (i32, i32),
    forward_cost: i64
}

#[derive(Eq, Hash, PartialEq)]
enum Heading {
    Straight,
    Left,
    Right,
}
fn main() {
    println!("day16 part 2");
    let mut myin = MyIn::new();
    let mut grid = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect::<Vec<char>>());
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

    let mut visited = vec![vec![HashMap::new(); grid[0].len()]; grid.len()];
    let mut costs = vec![vec![i64::MAX; grid[0].len()]; grid.len()];
    let min_path = find_min_path(
        &mut grid,
        Position {
            row: start_row as i32,
            col: start_col as i32,
            dir: (0, 1),
            forward_cost: 0,
        },
        &mut visited,
        i64::MAX,
        &mut costs
    );
    println!("------");
    let mut visited = vec![vec![HashMap::new(); grid[0].len()]; grid.len()];
    let mut costs = vec![vec![i64::MAX; grid[0].len()]; grid.len()];
    find_min_path(
        &mut grid,
        Position {
            row: start_row as i32,
            col: start_col as i32,
            dir: (0, 1),
            forward_cost: 0,
        },
        &mut visited,
        min_path,
        &mut costs,
    );
    print_grid(&grid);
    print_costs(&costs);
    println!("{}", min_path);
}

fn find_min_path(grid: &mut Vec<Vec<char>>, pos: Position, visited: &mut Vec<Vec<HashMap<(usize, usize), i64>>>, found_min: i64, costs: &mut Vec<Vec<i64>>) -> i64 {
    let row = pos.row as usize;
    let col = pos.col as usize;
    match grid[pos.row as usize][pos.col as usize] {
        '#' => i64::MAX,
        'E' => 0,
        _ =>
            {
                let mut min_cost = i64::MAX;
                /*
                let rotations: Vec<&Heading> = vec![&Straight, &Left, &Right].iter()
                    .map(|x| *x)
                    .filter(|x| pos.forward_cost <= *visited[row][col].get(*x).unwrap_or_else(|| &i64::MAX))
                    .collect();
                for rot in rotations.clone() {
                    visited[row][col].insert(rot, pos.forward_cost);
                }
                 */
                for rot in [&Straight, &Left, &Right].iter() {
                    let inc_cost = match rot {
                        Straight => 1,
                        _ => 1001,
                    };
                    let cost_to_dest = pos.forward_cost + inc_cost;
                    let new_pos = pos_rotate(&pos, &rot);
                    let dest_row = new_pos.row as usize;
                    let dest_col = new_pos.col as usize;
                    let dst = (dest_row, dest_col);
                    let prev_cost_to_dest = if visited[row][col].contains_key(&dst) {
                        *visited[row][col].get(&dst).unwrap()
                    } else {
                        i64::MAX
                    };
                    if cost_to_dest < prev_cost_to_dest {
                        visited[row][col].insert((dest_row, dest_col), cost_to_dest);
                        let mut cost = find_min_path(grid, new_pos, visited, found_min - inc_cost, costs);
                        if cost != i64::MAX {
                            cost += inc_cost;
                        }
                        if cost == found_min {
                            grid[row][col] = 'O';
                        }
                        min_cost = min(costs[row][col], cost);
                        if row == 7 && col == 5 {
                            println!("rc {} {} {} {}", row, col, dest_row, dest_col);
                            println!("{} {} {}", cost, min_cost, inc_cost);
                        }
                        costs[row][col] = min_cost
                    }
                }
                min_cost
            }
    }
}

fn pos_rotate(pos: &Position, rot: &Heading) -> Position {
    let new_dir = match rot {
        Straight => pos.dir,
        Left => (-pos.dir.1, pos.dir.0),
        Right => (pos.dir.1, -pos.dir.0),
    };
    Position {
        row: pos.row + new_dir.0,
        col: pos.col + new_dir.1,
        dir: new_dir,
        forward_cost: pos.forward_cost + match rot {
            Straight => 1,
            _ => 1001,
        }
    }
}

fn print_costs(costs: &Vec<Vec<i64>>) {
    println!("-------------");
    for row in costs {
        for c in row {
            if *c == i64::MAX {
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
