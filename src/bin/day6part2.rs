use bcr33d_aoc_2024::MyIn;
use std::collections::HashSet;

fn rotation_map(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        '<' => '^',
        'v' => '<',
        _ => panic!("Unknown direction: {}", dir),
    }
}

fn move_delta(dir: char) -> Delta {
    match dir {
        '^' => Delta { drow: -1, dcol: 0 },
        '>' => Delta { drow: 0, dcol: 1 },
        '<' => Delta { drow: 0, dcol: -1 },
        'v' => Delta { drow: 1, dcol: 0 },
        _ => panic!("Unknown direction: {}", dir),
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct Heading {
    row: isize,
    col: isize,
    dir: char,
}

impl Heading {
    fn rotate(&self) -> Heading {
        Heading {
            row: self.row,
            col: self.col,
            dir: rotation_map(self.dir),
        }
    }

    fn on_grid(&self, grid: &Vec<Vec<char>>) -> bool {
        self.row >= 0
            && self.row < grid.len() as isize
            && self.col >= 0
            && self.col < grid[0].len() as isize
    }

    fn on_wall(&self, grid: &Vec<Vec<char>>) -> bool {
        self.on_grid(grid) && grid[self.row as usize][self.col as usize] == '#'
    }

    fn insert_dir(&self, dirs: &mut Vec<Vec<HashSet<char>>>) {
        dirs[self.row as usize][self.col as usize].insert(self.dir);
    }

    fn fill_behind(&self, grid: &Vec<Vec<char>>, dirs: &mut Vec<Vec<HashSet<char>>>) {
        let mut cur_heading = self.clone();
        while cur_heading.on_grid(grid) && !cur_heading.on_wall(grid) {
            cur_heading.insert_dir(dirs);
            cur_heading = cur_heading.move_heading(false);
        }
    }

    fn move_heading(&self, forward: bool) -> Heading {
        let d = move_delta(self.dir);
        Heading {
            row: self.row + d.drow * if forward { 1 } else { -1 },
            col: self.col + d.dcol * if forward { 1 } else { -1 },
            dir: self.dir,
        }
    }
}

struct Delta {
    drow: isize,
    dcol: isize,
}

fn main() {
    println!("day6 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect());
    }

    let mut dirs = vec![vec![HashSet::<char>::new(); grid[0].len()]; grid.len()];

    let starting_heading = 'outer: loop {
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '^' {
                    break 'outer Heading {
                        row: r as isize,
                        col: c as isize,
                        dir: '^',
                    };
                }
            }
        }
    };

    let mut heading = starting_heading.clone();
    heading.fill_behind(&grid, &mut dirs);
    while heading.on_grid(&grid) {
        dirs[heading.row as usize][heading.col as usize].insert(heading.dir);
        let mut new_heading = heading.move_heading(true);
        if new_heading.on_wall(&grid) {
            new_heading = heading.rotate();
            new_heading.fill_behind(&grid, &mut dirs);
        } else {
            let rot = rotation_map(heading.dir);
            let poss_dirs = &dirs[heading.row as usize][heading.col as usize];
            if poss_dirs.contains(&rot) {
                total += 1;
            }
        }
        heading = new_heading
    }

    print_dirs(&dirs);

    println!("{}", total);
}

fn print_dirs(dirs: &Vec<Vec<HashSet<char>>>) {
    for r in 0..dirs.len() {
        for c in 0..dirs[0].len() {
            let s = dirs[r][c].iter().collect::<String>();
            print!("{:<5}", if s.is_empty() { ".".to_string() } else { s });
        }
        println!();
    }
}
