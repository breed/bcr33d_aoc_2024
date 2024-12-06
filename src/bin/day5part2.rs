use bcr33d_aoc_2024::MyIn;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("day5 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    let mut after: HashMap<i32, HashSet<i32>> = HashMap::new();
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let parts: Vec<_> = line
            .split("|")
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect();
        match after.get_mut(&parts[0]) {
            Some(hs) => {
                hs.insert(parts[1]);
            }
            None => {
                let mut hs = HashSet::new();
                hs.insert(parts[1]);
                after.insert(parts[0], hs);
            }
        }
    }
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut parts: Vec<_> = line.split(",").map(|s| s.parse().unwrap_or(0)).collect();
        let mut good = true;
        'outer: for i in 0..parts.len() {
            if let Some(hs) = after.get(&parts[i]) {
                // println!("Checking {:#?} against {:?}", &parts[0..i], hs);
                for j in 0..i {
                    if hs.contains(&parts[j]) {
                        good = false;
                        break 'outer;
                    }
                }
            }
        }
        if !good {
            parts.sort_by(|a, b| {
                if let Some(hs) = after.get(a) {
                    if hs.contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                } else if let Some(hs) = after.get(b) {
                    if hs.contains(a) {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            });
            let mid = parts[parts.len() / 2] as i64;
            total += mid;
        }
    }
    println!("{}", total);
}
