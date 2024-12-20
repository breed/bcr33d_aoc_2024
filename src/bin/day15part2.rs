use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day15 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut grid = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(
            line.chars()
                .flat_map(|c| match c {
                    '@' => ['@', '.'],
                    'O' => ['[', ']'],
                    _ => [c, c],
                })
                .collect::<Vec<char>>(),
        );
    }

    // find the robot
    let mut start_row = 0;
    let mut start_col = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                start_row = row;
                start_col = col;
                println!("found @ at {} {}", row, col);
                break;
            }
        }
    }
    println!("found @ at {} {}", start_row, start_col);

    let mut row = start_row;
    let mut col = start_col;

    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let moves = line.chars().collect::<Vec<char>>();
        for m in moves {
            let delta = match m {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => {
                    panic!("bad move");
                }
            };
            print_grid(&grid);
            if try_move(&mut grid, '@', row, col, delta, false) {
                try_move(&mut grid, '@', row, col, delta, true);
                grid[row][col] = '.';
                (row, col) = apply_move(row, col, delta);
                grid[row][col] = '@';
            }
        }
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '[' {
                total += 100 * row as i64 + col as i64;
            }
        }
    }

    println!("{}", total);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn try_move(
    grid: &mut Vec<Vec<char>>,
    new_c: char,
    row: usize,
    col: usize,
    delta: (i32, i32),
    do_it: bool,
) -> bool {
    let (new_row, new_col) = apply_move(row, col, delta);
    let c = grid[new_row][new_col];
    match c {
        '[' => {
            if delta.0 == 0 && try_move(grid, '[', new_row, new_col, delta, do_it) {
                if do_it {
                    grid[new_row][new_col] = new_c;
                }
                true
            } else if delta.1 == 0 && try_move_box_up(grid, new_row, new_col, delta.0, do_it) {
                true
            } else {
                false
            }
        }
        // yuck dup code. quick and dirty...
        ']' => {
            if delta.0 == 0 && try_move(grid, ']', new_row, new_col, delta, do_it) {
                if do_it {
                    grid[new_row][new_col] = new_c;
                }
                true
            } else if delta.1 == 0 && try_move_box_up(grid, new_row, new_col - 1, delta.0, do_it) {
                true
            } else {
                false
            }
        }
        '.' => {
            if do_it {
                grid[new_row][new_col] = new_c;
            }
            true
        }
        '#' => false,
        _ => {
            panic!("bad char {}", c);
        }
    }
}

// row,col should be the left side of the box
fn try_move_box_up(
    grid: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    delta: i32,
    do_it: bool,
) -> bool {
    let new_row = row as i32 + delta;
    let can_move = match grid[new_row as usize][col] {
        '.' => true,
        '[' => try_move_box_up(grid, new_row as usize, col, delta, do_it),
        ']' => try_move_box_up(grid, new_row as usize, col - 1, delta, do_it),
        _ => false,
    } && match grid[new_row as usize][col + 1] {
        '.' => true,
        '[' => try_move_box_up(grid, new_row as usize, col + 1, delta, do_it),
        ']' => true, // this is checked by the previous clause
        _ => false,
    };
    if can_move {
        if do_it {
            grid[row][col] = '.';
            grid[row][col + 1] = '.';
            grid[new_row as usize][col] = '[';
            grid[new_row as usize][col + 1] = ']';
        }
        true
    } else {
        false
    }
}

fn apply_move(old_row: usize, old_col: usize, delta: (i32, i32)) -> (usize, usize) {
    (
        (old_row as i32 + delta.0) as usize,
        (old_col as i32 + delta.1) as usize,
    )
}
