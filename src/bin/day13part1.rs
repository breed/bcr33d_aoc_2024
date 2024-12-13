use std::cmp::min;
use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day13 part 1");
    let mut myin = MyIn::new();
    let mut total: usize = 0;
    let mut line = 0;
    loop {
        let button_a = myin.read_line();
        if button_a.is_empty() {
            break;
        }
        let (adx, ady) = extract_xy(button_a);
        let button_b = myin.read_line();
        let (bdx, bdy) = extract_xy(button_b);
        let dest = myin.read_line();
        let (destx, desty) = extract_xy(dest);
        let _ = myin.read_line();
        // takes so long to run! make sure progress is happening
        println!("{}", line);
        line += 1;

        let mut grid = vec![vec![usize::max_value(); desty+1]; destx+1];
        let cost = explore(&mut grid, 0, 0, 0,  adx, ady, 0, bdx, bdy, 0, destx, desty);
        if cost < usize::max_value() {
            total += cost;
        }
    }
    println!("{}", total);
}

fn explore(grid: &mut Vec<Vec<usize>>, cost: usize, x: usize, y: usize, adx: usize, ady: usize, apress: i32, bdx: usize, bdy: usize, bpress: i32, destx: usize, desty: usize) -> usize {
    if (x >= grid.len()) || (y >= grid[0].len()) {
        return usize::max_value();
    }
    if cost >= grid[x][y] {
        return usize::max_value();
    }
    if grid[x][y] != usize::max_value() {
        println!("FOUUND DUP!!!!);")
    }
    grid[x][y] = cost;
    if x == destx && y == desty {
        return cost;
    }
    let mut acost = usize::max_value();
    let mut bcost = usize::max_value();
    if apress < 100 {
         acost = explore(grid, cost + 3, x + adx, y + ady, adx, ady, apress + 1, bdx, bdy, bpress, destx, desty);
    }
    if bpress < 100 {
        bcost = explore(grid, cost + 1, x + bdx, y + bdy, adx, ady, apress, bdx, bdy, bpress + 1, destx, desty);
    }
    min(acost, bcost)
}

fn extract_xy(line: String) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    line.split(" ").for_each(|w| {
        if w.starts_with("X") {
            x = w[2..w.len()-1].parse::<usize>().unwrap();
        } else if w.starts_with("Y"){
            y = w[2..].parse::<usize>().unwrap();
        }
    });
    (x,y)
}