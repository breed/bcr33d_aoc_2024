use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day25 part 1");
    let mut myin = MyIn::new();
    let mut matches = 0;
    let mut holes: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    let mut last_saw_key = false;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        if line == "#####" {
            if false && last_saw_key { // evidently we pair with all holes and keys, not just the sets
                count_matches(&mut matches, &mut holes, &mut keys);
            }
            last_saw_key = false;
            let mut hole = [0; 5];

            for _ in 0..6 {
                let line = myin.read_line();
                let mut chars = line.chars();
                for j in 0..5 {
                    if chars.next().unwrap() == '#' {
                        hole[j] += 1;
                    }
                }
            }
            holes.push(hole);
        } else {
            last_saw_key = true;
            let mut key = [0; 5];
            for _ in 0..5 {
                let line = myin.read_line();
                let mut chars = line.chars();
                for j in 0..5 {
                    if chars.next().unwrap() == '#' {
                        key[j] += 1;
                    }
                }
            }
            myin.read_line(); // .....
            keys.push(key);
        }

        myin.read_line();
    }
    count_matches(&mut matches, &mut holes, &mut keys);

    println!("{}", matches);
}

fn count_matches(matches: &mut i32, holes: &mut Vec<[u8; 5]>, keys: &mut Vec<[u8; 5]>) {
    let orig_holes = &holes;
    let orig_keys = &keys;
    for hole in orig_holes.iter() {
        for key in orig_keys.iter() {
            let mut matched = true;
            for i in 0..5 {
                if hole[i] + key[i] > 5 {
                    matched = false;
                    break;
                }
            }
            if matched {
                *matches += 1;
            }
        }
    }
    *holes = Vec::new();
    *keys = Vec::new();
}
