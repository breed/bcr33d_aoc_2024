use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day7 part 1");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(":");
        let answer = parts.next().unwrap().parse().unwrap_or(0);
        let parts = parts.next().unwrap().split(" ");
        let nums : Vec<i32> = parts.map(|s| s.parse::<i32>().unwrap_or(0)).collect();
        if find_op(&nums, 1, nums[0] as i64, &answer, '*') || find_op(&nums, 1, nums[0] as i64, &answer, '+') {
            total += answer;
        };
    }

    println!("{}", total);
}

fn find_op(nums: &Vec<i32>, i: usize, current: i64, answer: &i64, op: char) -> bool {
    if i == nums.len() {
        return current == *answer;
    }
    let result = if op == '*' {
        current as i64 * nums[i] as i64
    } else {
        current as i64 + nums[i] as i64
    };
    if result > *answer {
        return false;
    }
    find_op(nums, i + 1, result, answer, '*') || find_op(nums, i + 1, result, answer, '+')
}