use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day4 part 2");
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
    for i in 0..grid.len() as i32 {
        for j in 0..grid[i as usize].len() as i32 {
            if grid[i as usize][j as usize] == b'A' {
                if check(get(&grid, i - 1, j + 1), get(&grid, i + 1, j - 1))
                    && check(get(&grid, i - 1, j - 1), get(&grid, i + 1, j + 1))
                {
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}

fn check(m: u8, s: u8) -> bool {
    (m == b'M' && s == b'S') || (m == b'S' && s == b'M')
}

fn get(grid: &Vec<Vec<u8>>, i: i32, j: i32) -> u8 {
    let i = i as usize;
    let j = j as usize;
    if i < grid.len() && j < grid[i].len() {
        grid[i][j]
    } else {
        b'*'
    }
}
