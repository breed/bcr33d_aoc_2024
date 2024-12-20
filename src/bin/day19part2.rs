use bcr33d_aoc_2024::MyIn;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut myin = MyIn::new();
    let line = myin.read_line();
    let patterns = HashSet::from_iter(line.split(", ").collect::<Vec<&str>>());
    myin.read_line();
    let mut total = 0;
    let mut line_num = 0;
    loop {
        line_num += 1;
        println!("line: {}", line_num);
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut memo = HashMap::new();
        for pattern in patterns.iter() {
            if line.starts_with(pattern) {
                let rest = line.strip_prefix(pattern).unwrap();
                total += pattern_match(rest, &patterns, &mut memo);
            }
        }
    }
    println!("total: {}", total);
}

fn pattern_match(line: &str, patterns: &HashSet<&str>, memo: &mut HashMap<String, u64>) -> u64 {
    if line.is_empty() {
        return 1;
    }
    if memo.contains_key(line) {
        return memo[line];
    }
    let mut total = 0;
    for pattern in patterns.iter() {
        if line.starts_with(pattern) {
            let rest = line.strip_prefix(pattern).unwrap();
            total += pattern_match(rest, &patterns, memo);
        }
    }
    memo.insert(line.to_string(), total);
    total
}
