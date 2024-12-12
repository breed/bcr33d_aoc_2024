use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day12 part 1");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut grid = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut regions = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut region_num = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c != '.' {
                dfs(&mut grid, row, col, &mut regions, c, region_num);
                region_num += 1;
            }
        }
    }
    let mut region_counts = vec![0; region_num as usize];
    let mut region_param = vec![0; region_num as usize];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let region = regions[row][col] as usize;
            region_counts[region] += 1;
            for moves in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let new_row = (row as i32 + moves.0) as usize;
                let new_col = (col as i32 + moves.1) as usize;
                // we take advantage that usize underflows to a big number
                if new_row >= grid.len()
                    || new_col >= grid[new_row].len()
                    || regions[new_row][new_col] != region as i32
                {
                    region_param[region] += 1;
                }
            }
        }
    }
    for i in 0..region_num as usize {
        println!(
            "region {} has {} cells and {} params",
            i, region_counts[i], region_param[i]
        );
        total += region_counts[i] * region_param[i];
    }
    println!("{}", total);
}

fn dfs(
    grid: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    regions: &mut Vec<Vec<i32>>,
    c: char,
    region_num: i32,
) {
    if row >= grid.len() || col >= grid[row].len() {
        return;
    }
    if grid[row][col] != c {
        return;
    }
    grid[row][col] = '.';
    regions[row][col] = region_num;
    for moves in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        let new_row = row as isize + moves.0;
        let new_col = col as isize + moves.1;
        dfs(
            grid,
            new_row as usize,
            new_col as usize,
            regions,
            c,
            region_num,
        );
    }
}
