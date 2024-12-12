use bcr33d_aoc_2024::MyIn;
use std::collections::HashMap;

fn main() {
    println!("day11 part 2");
    let mut myin = MyIn::new();
    let line = myin.read_line();
    let mut nums: HashMap<String, i64> = HashMap::new();
    line.split(" ").for_each(|s| {
        *nums.entry(s.to_string()).or_default() += 1;
    });
    for _ in 0..75 {
        let mut max_size = 0;
        let mut new_nums: HashMap<String, i64> = HashMap::new();
        nums.iter().for_each(|(k, v)| {
            if k.eq("0") {
                *new_nums.entry("1".to_string()).or_default() += v;
            } else if k.to_string().len() % 2 == 0 {
                let strlen = k.to_string().len();
                let half = strlen / 2;
                *new_nums.entry(k[..half].to_string()).or_default() += v;
                *new_nums
                    .entry(k[half..].parse::<i64>().unwrap().to_string())
                    .or_default() += v;
            } else {
                let big = (k.parse::<i64>().unwrap() * 2024).to_string();
                if max_size < big.len() {
                    max_size = big.len();
                }
                *new_nums.entry(big).or_default() += v;
            }
        });
        nums = new_nums;
    }
    println!("{}", nums.iter().map(|(_, v)| v).sum::<i64>());
}
