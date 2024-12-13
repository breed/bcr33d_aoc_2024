use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day13 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
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
        let (mut destx, mut desty) = extract_xy(dest);
        destx += 10000000000000;
        desty += 10000000000000;

        // if this happens, we will get the wrong answer and probably panic with a
        // divide by zero error (luckily it doesn't happen for the given input)
        if adx * bdy == bdx * ady {
            println!("RATIO!!! {},{} {},{}", adx, ady, bdx, bdy);
        }
        line += 1;

        // cramer's rule, although i forgot about cramer's rule and derived it from scratch...
        let mut b = desty * adx;
        b -= destx * ady;
        b /= adx * bdy - ady * bdx;

        // the simple substition to calculate a using b
        let mut a = destx - b * bdx;
        a /= adx;

        // i miss typed the submitted answer, so i thought my solution was wrong
        // i calculate a using cramer's rule to verify my solution
        let mut alta = destx * bdy;
        alta -= desty * bdx;
        alta /= adx * bdy - ady * bdx;

        // more debugging of my correct solution because i thought the answer was wrong
        // (due to a typo in the submission)
        if a < 0 || b < 0 {
            println!("!!!!!!!!!NEG {} {}!!!!!", a, b);
        }
        if a != alta {
            // turns out this is okay if a is not the answer... it's just a different
            // approximation of a wrong answer for a solution that requires floating point
            println!("!!!!!!!!!ALERT {} {}", a, alta);
        }

        // debugging to make sure that the two calculations of a always
        // come up with the same answer
        if a * adx + b * bdx == destx && a * ady + b * bdy == desty {
            total += a * 3 + b;
            println!("***WIN {} {} {}", a, b, total);
        }

        if alta * adx + b * bdx == destx && alta * ady + b * bdy == desty {
            println!("***ALT WIN {} {}", a, b);
        }

        let extra = myin.read_line();
        if !extra.is_empty() {
            break;
        }
    }
    println!("{}", total);
}

fn extract_xy(line: String) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;
    line.split(" ").for_each(|w| {
        if w.starts_with("X") {
            x = w[2..w.len() - 1].parse().unwrap();
        } else if w.starts_with("Y") {
            y = w[2..].parse().unwrap();
        }
    });
    (x, y)
}
