use regex::Regex;
use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day3 part 1");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let re = Regex::new(
            r"((?<mul>mul)\((?<a>[0-9]+),(?<b>[0-9]+)\))|((?<do>do)\(\))|((?<dont>don\'t)\(\))",
        )
        .unwrap();
        for parts in re.captures_iter(&line) {
            if parts.name("mul").is_some() {
                let a = parts.name("a").unwrap().as_str().parse::<i64>().unwrap();
                let b = parts.name("b").unwrap().as_str().parse::<i64>().unwrap();
                total += a * b;
            }
        }
    }
    println!("{}", total);
}
