use std::collections::HashSet;
use bcr33d_aoc_2024::MyIn;

fn main() {
    let mut myin = MyIn::new();
    let line = myin.read_line();
    let patterns = HashSet::from_iter(line.split(", ").collect::<Vec<&str>>());
    myin.read_line();
    let mut total = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        for pattern in patterns.iter() {
            if line.starts_with(pattern) {
                let rest = line.strip_prefix(pattern).unwrap();
                if pattern_match(rest, &patterns) {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("total: {}", total);
}

fn pattern_match(line: &str, patterns: &HashSet<&str>) -> bool {
    if line.is_empty() {
        return true;
    }
    let mut found = false;
    for pattern in patterns.iter() {
        if line.starts_with(pattern) {
            let rest = line.strip_prefix(pattern).unwrap();
            if pattern_match(rest, &patterns) {
                found = true;
                break;
            }
        }
    }
    found
}