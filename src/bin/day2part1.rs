use bcr33d_aoc_2024::MyIn;

fn main() {
    let mut myin = MyIn::new();
    let mut safe_count = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut nums = Vec::new();
        for word in line.split_whitespace() {
            let num = word.parse::<i64>().unwrap();
            nums.push(num);
        }

            if check(&nums) == -1 {
                safe_count += 1;
                break;
            }
    }
    println!("{}", safe_count);
}

fn check(nums: &Vec<i64>) -> i32 {
    let mut prevsig = 0;
    for i in 0..nums.len()-1 {
        let diff = nums[i+1] - nums[i];
        if diff.abs() < 1 || diff.abs() > 3 || nums[i] == 0 {
            return i as i32;
        }
        if prevsig != 0 && diff.signum() != prevsig {
            return i as i32;
        }
        prevsig = diff.signum();
    }
    -1
}