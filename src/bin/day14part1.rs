use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day14 part 1");
    let mut myin = MyIn::new();
    let mut quad = vec![0;4];
    let max_x = 101;
    let max_y = 103;
    let mid_x = max_x / 2;
    let mid_y = max_y / 2;
    loop {
        let pos_line = myin.read_word();
        if pos_line.is_empty() {
            break;
        }
        let (x,y) = extract_xy(pos_line);
        let vel_line = myin.read_word();
        let (dx,dy) = extract_xy(vel_line);
        let final_x = (x + dx * 100).rem_euclid(max_x);
        let final_y = (y + dy * 100).rem_euclid(max_y);
        let q = if final_x < mid_x {
            if final_y < mid_y {
                0
            } else if final_y > mid_y {
                1
            }
            else {
                -1
            }
        } else if final_x > mid_x {
            if final_y < mid_y {
                2
            } else if final_y > mid_y {
                3
            } else {
                -1
            }
        } else {
            -1
        };
        println!("({},{}) ended up in ({},{}) {}", x, y, final_x, final_y, q);
        if q != -1 {
            quad[q as usize] += 1;
        }
    }
    quad.iter().for_each(|x| println!("{}", x));
    println!("{}", quad.iter().fold(1, |acc,x| acc*x));
}

fn extract_xy(line: String) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let parts = line[2..].split(",");
    let mut it = parts.into_iter();
    (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
}
