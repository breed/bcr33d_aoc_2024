use std::thread::sleep;
use bcr33d_aoc_2024::MyIn;

struct RobotPos {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}
fn main() {
    println!("day14 part 1");
    let mut myin = MyIn::new();
    let max_x = 101;
    let max_y = 103;
    let mut robots = Vec::new();
    loop {
        let pos_line = myin.read_word();
        if pos_line.is_empty() {
            break;
        }
        let (x, y) = extract_xy(pos_line);
        let vel_line = myin.read_word();
        let (dx, dy) = extract_xy(vel_line);
        let robot = RobotPos { x, y, dx, dy };
        robots.push(robot);
    }
    for sec in 0..100000 {
        let mut grid = vec![vec![' '; max_x as usize]; max_y as usize];
        let mut possible = true;
        robots.iter_mut().for_each(|r| {
            let mut c = grid[r.y as usize][r.x as usize];
            c = if c == ' ' { '*' } else { '#' };
            if c == '#' { possible = false;}
            grid[r.y as usize][r.x as usize] = c;
            r.x = (r.x + r.dx).rem_euclid(max_x);
            r.y = (r.y + r.dy).rem_euclid(max_y);
        });
        if !possible {
            continue
        }
        println!("\033[2J ----- sec {} -----", sec);
        for row in 0..max_y as usize{
            for col in 0..max_x as usize{
                print!("{}", grid[row][col]);
            }
            println!();
        }
        println!("----- sec {} -----", sec);
        sleep(std::time::Duration::from_secs(5));
    }
}

fn extract_xy(line: String) -> (i32, i32) {
    let parts = line[2..].split(",");
    let mut it = parts.into_iter();
    (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
}
