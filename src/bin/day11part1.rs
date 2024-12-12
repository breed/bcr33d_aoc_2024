use std::cmp::max;
use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day11 part 1");
    let mut myin = MyIn::new();
    let line = myin.read_line();
    let mut nums : Vec<String> = line.split(" ").map(|s| {s.to_string()}).collect();
    for i in 0..25 {
        let mut max_size = 0;
        nums = nums.iter().flat_map(|x| {
            if x.eq("0") {
                vec!["1".to_string(); 1]
            } else if x.to_string().len() % 2 == 0 {
                let strlen = x.to_string().len();
                let half = strlen / 2;
                vec![x[..half].to_string(), x[half..].parse::<i32>().unwrap().to_string()]
            }  else {
                let big = (x.parse::<i64>().unwrap() * 2024).to_string();
                if max_size < big.len() { max_size = big.len(); }
                vec![big]
            }
        }).collect();
        println!("{} {} max_size: {}", i, nums.len(), max_size);
    }
    println!("{}", nums.len());
}