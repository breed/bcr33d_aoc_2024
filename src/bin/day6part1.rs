use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day6 part 1");
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

    loop {
        let origx = x;
        let origy = y;
        let c = grid[y as usize][x as usize];
        grid[y as usize][x as usize] = b'X';
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
                b'^' => b'>',
                b'v' => b'<',
                b'<' => b'^',
                b'>' => b'v',
                _ => panic!("bad dir"),
            }
        } else {
            grid[y as usize][x as usize] = c;
            //print_grid(&grid);
        }
    }
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'X' {
                total += 1;
            }
        }
    }

    println!("{}", total);
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
