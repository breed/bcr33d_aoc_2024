use std::collections::HashSet;
use std::ops::Range;
use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day6 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut grid = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(line.into_bytes());
    }
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    'outer:
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'^' || grid[row][col] == b'v' || grid[row][col] == b'<' || grid[row][col] == b'>' {
                x = col as i32;
                y = row as i32;
                break 'outer;
            }
        }
    }

    let mut dirs = vec![vec![HashSet::new(); grid[0].len()]; grid.len()];

    let startx = x;
    let starty = y;

    grid[y as usize][x as usize] = b'v';
    print_grid(&grid);
    explore(&mut total, &mut grid, x, y, &mut dirs, true);
    x = startx;
    y = starty;
    grid[y as usize][x as usize] = b'^';
    total = 0;
    explore(&mut total, &mut grid, x, y, &mut dirs, false);
    println!("{}", total);
}

fn explore(total: &mut i64, grid: &mut Vec<Vec<u8>>, mut x: i32, mut y: i32, dirs: &mut Vec<Vec<HashSet<u8>>>, backwards: bool) {
    loop {
        let origx = x;
        let origy = y;
        let c = grid[y as usize][x as usize];
        match c {
            b'^' => {
                y -= 1;
            }
            b'v' => {
                y += 1;
            }
            b'<' => {
                x -= 1;
            }
            b'>' => {
                x += 1;
            }
            _ => {
                panic!("bad dir");
            }
        }
        if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[y as usize].len() as i32 {
            break;
        }
        if grid[y as usize][x as usize] == b'#' {
            x = origx;
            y = origy;
            grid[y as usize][x as usize] = match c {
                b'^' => {
                    fill(dirs, &(0..x), &(y..y+1), b'>', backwards);
                    b'>'
                }
                b'v' => {
                    fill(dirs, &(x..grid[0].len() as i32), &(y..y+1), b'<', backwards);
                    b'<'
                },
                b'<' => {
                    fill(dirs, &(x..x+1), &(y..grid.len() as i32), b'^', backwards);
                    b'^'
                },
                b'>' => {
                    fill(dirs, &(x..x+1), &(0..y), b'v', backwards);
                    b'v'
                },
                _ => panic!("bad dir"),
            }
        } else {
            grid[y as usize][x as usize] = c;
           store_dir(&mut dirs[y as usize][x as usize], c, backwards);
           .print_grid(&grid);
            match c {
                b'^' => {
                    if dirs_has(&dirs, origx + 1, origy, b'>') {
                        println!("at {},{} looking at {}", x, y, c as char);
                        *total += 1;
                    }
                }
                b'v' => {
                    if dirs_has(&dirs, origx - 1, origy, b'<') {
                        println!("at {},{} looking at {}", x, y, c as char);
                        *total += 1;
                    }
                }
                b'<' => {
                    if dirs_has(&dirs, origx, origy - 1, b'^') {
                        println!("at {},{} looking at {}", x, y, c as char);
                        *total += 1;
                    }
                }
                b'>' => {
                    if dirs_has(&dirs, origx, origy + 1, b'v') {
                        println!("at {},{} looking at {}", x, y, c as char);
                        *total += 1;
                    }
                }
                _ => {
                    panic!("bad dir");
                }
            }
        }
    }
}

fn char_map(c: u8, backwards: bool) -> u8 {
    if backwards {
        match c {
            b'^' => b'v',
            b'v' => b'^',
            b'<' => b'>',
            b'>' => b'<',
            _ => panic!("bad dir"),
        }
    } else {
        c
    }
}

fn store_dir(dir: &mut HashSet<u8>, c: u8, backwards: bool) {
    dir.insert(
        char_map(c, backwards)
    );
}

fn fill(dir: &mut Vec<Vec<HashSet<u8>>>, cols: &Range<i32>, rows: &Range<i32>, mut c: u8, backwards: bool) {
    c = char_map(c, backwards);
    for col in cols.clone().into_iter() {
        for row in rows.clone().into_iter() {
            dir[row as usize][col as usize].insert(c);
        }
    }
}

fn dirs_has(dirs: &Vec<Vec<HashSet<u8>>>, x: i32, y: i32, c: u8) -> bool {
    if x >= 0 && x < dirs[0].len() as i32 && y >= 0 && y < dirs.len() as i32 {
        println!("checking {:?} {},{}", dirs[y as usize][x as usize], x, y);
        dirs[y as usize][x as usize].contains(&c)
    } else {
        println!("nothing");
        false
    }
}

fn print_grid(p0: &Vec<Vec<u8>>) {
    println!("-------------");
    for row in p0 {
        for c in row {
            print!("{}", *c as char);
        }
        println!();
    }
}
