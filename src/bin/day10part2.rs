use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day9 part 1");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut grid = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(
            line.into_bytes()
                .iter()
                .map(|&x| x - b'0')
                .collect::<Vec<u8>>(),
        );
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 0 {
                let trails = explore(&grid, row, col);
                println!("{} {} have {}", row, col, trails);
                total += trails;
            }
        }
    }
    println!("{}", total);
}

fn explore(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> i64 {
    let mut explored = vec![vec![false; grid[0].len()]; grid.len()];
    dfs(grid, &mut explored, row, col)
}

fn dfs(grid: &Vec<Vec<u8>>, explored: &mut Vec<Vec<bool>>, row: usize, col: usize) -> i64 {
    if explored[row][col] {
        return 0;
    }
    let mut total = 0;
    let height = grid[row][col];
    if height == 9 {
        return 1;
    }
    for moves in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        let new_row = row as isize + moves.0;
        let new_col = col as isize + moves.1;
        if in_grid(grid, new_row, new_col)
            && !explored[new_row as usize][new_col as usize]
            && grid[new_row as usize][new_col as usize] == height + 1
        {
            total += dfs(grid, explored, new_row as usize, new_col as usize);
        }
    }
    total
}

fn in_grid(grid: &Vec<Vec<u8>>, row: isize, col: isize) -> bool {
    row >= 0 && col >= 0 && row < grid.len() as isize && col < grid[0].len() as isize
}
