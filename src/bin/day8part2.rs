use bcr33d_aoc_2024::MyIn;
use std::collections::{HashMap, HashSet};

struct RowCol {
    row: isize, col: isize,
}

fn main() {
    println!("day8 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut grid: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut letters: HashMap<char, Vec<RowCol>> = HashMap::new();

    let mut rows = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(vec![HashSet::new();line.len()]);
        for (i, c) in line.chars().enumerate() {
            if c != '.' {
                letters.entry(c).or_insert(Vec::new()).push(RowCol{row: rows as isize, col: i as isize});
            }
        }
        rows += 1;
    }
    for (c, row_cols) in letters.iter() {
        for i in 0..row_cols.len() {
            for j in i+1..row_cols.len() {
                let drow = row_cols[j].row - row_cols[i].row;
                let dcol = row_cols[j].col - row_cols[i].col;
                let mut new_irow = row_cols[i].row;
                let mut new_icol = row_cols[i].col;
                let mut new_jrow = row_cols[j].row;
                let mut new_jcol = row_cols[j].col;
                while in_grid(&grid, new_irow, new_icol) {
                    grid[new_irow as usize][new_icol as usize].insert(*c);
                    new_irow -= drow;
                    new_icol -= dcol;
                }
                while in_grid(&grid, new_jrow, new_jcol) {
                    grid[new_jrow as usize][new_jcol as usize].insert(*c);
                    new_jrow += drow;
                    new_jcol += dcol;
                }
            }
        }
    }
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col].len() >= 1 {
                total += 1;
            }
        }
    }
    print_grid(&grid);
    println!("{}", total);
}

fn print_grid(grid: &Vec<Vec<HashSet<char>>>) {
    for row in grid {
        for col in row {
            let str = if col.len() == 0 { ".".to_string() } else { col.iter().collect::<String>() };
            print!("{:<5}", &str);
        }
        println!();
    }
}

fn in_grid(grid: &Vec<Vec<HashSet<char>>>, row: isize, col: isize) -> bool {
    (row as usize) < grid.len() && (col as usize) < grid[0].len()
}
