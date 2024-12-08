use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day7 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(":");
        let answer = parts.next().unwrap().parse().unwrap_or(0);
        let parts = parts.next().unwrap().trim().split(" ");
        let nums : Vec<i32> = parts.map(|s| s.parse::<i32>().unwrap_or(0)).collect();
        println!("{:?}", nums);
        if calc(nums[0] as i64, &nums, 1, answer) {
            total += answer;
        }
    }

    println!("{}", total);
}

fn calc(current: i64, nums: &Vec<i32>, i: usize, answer: i64) -> bool {
    if i == nums.len() {
        return current == answer;
    }

    if current > answer { return false }

    let current_num = nums[i] as i64;
    calc(current * current_num, nums, i + 1, answer) || calc(current + current_num, nums, i + 1, answer) || calc(append(current, current_num), nums, i + 1, answer)
}

fn append(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}