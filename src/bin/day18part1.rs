use std::collections::VecDeque;
use std::ptr::read;
use bcr33d_aoc_2024::MyIn;

struct Step {
    x: i32,
    y: i32,
    step: i32,
}

const MAX_DIM: usize = 71;

fn main() {
    let mut myin = MyIn::new();
    let mut grid = vec![vec!['.'; MAX_DIM]; MAX_DIM];
    for _ in 0..1024 {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let prog = line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        grid[prog[1]][prog[0]] = '#';
    }

    let mut queue = VecDeque::new();

    queue.push_back(Step{x: 0, y: 0, step: 0});
    let mut final_step = Step{x: 70, y: 70, step: 0};
    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        let x = step.x;
        let y = step.y;
        let s = step.step;
        if x == (MAX_DIM - 1) as i32 && y == (MAX_DIM - 1) as i32 {
            final_step = step;
            break;
        }
        if y >= 0 && x >= 0 && y < MAX_DIM as i32 && x < MAX_DIM as i32 && grid[y as usize][x as usize] == '.' {
            grid[y as usize][x as usize] = 'O';
            for m in [(1,0), (-1,0), (0,1), (0,-1)].iter() {
                queue.push_back(Step{x: x + m.0, y: y + m.1, step: s + 1});
            }
        }
    }
    for row in 0..MAX_DIM {
        for col in 0..MAX_DIM {
            print!("{}", grid[row][col]);
        }
        println!();
    }
    println!("found at {} {} in {} steps", final_step.x, final_step.y, final_step.step);
    println!("finished")
}