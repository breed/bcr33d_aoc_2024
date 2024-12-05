use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day4 part 1");
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
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'X' {
                let rest = "MAS".as_bytes();
                if check(&grid, i, j, 1, 0, rest) {
                    total += 1;
                }
                if check(&grid, i, j, -1, 0, rest) {
                    total += 1;
                }
                if check(&grid, i, j, 0, 1, rest) {
                    total += 1;
                }
                if check(&grid, i, j, 0, -1, rest) {
                    total += 1;
                }
                if check(&grid, i, j, 1, 1, rest) {
                    total += 1;
                }
                if check(&grid, i, j, 1, -1, rest) {
                    total += 1;
                }
                if check(&grid, i, j, -1, 1, rest) {
                    total += 1;
                }
                if check(&grid, i, j, -1, -1, rest) {
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}

fn check(
    grid: &Vec<Vec<u8>>,
    mut i: usize,
    mut j: usize,
    inc_i: i32,
    inc_j: i32,
    rest: &[u8],
) -> bool {
    if rest.is_empty() {
        return true;
    }
    i = (i as i32 + inc_i) as usize;
    j = (j as i32 + inc_j) as usize;
    i < grid.len()
        && j < grid[i].len()
        && grid[i][j] == rest[0]
        && check(grid, i, j, inc_i, inc_j, &rest[1..])
}
