use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day9 part 1");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;

    let line = myin.read_line();
    let map: Vec<char> = line.chars().collect();
    let mut blocks = Vec::new();
    for i in (0..map.len()).step_by(2) {
        let block_num = (i / 2) as i32;
        let count = map[i] as i32 - '0' as i32;
        let free_count = if map.len() == i+1 {0} else {map[i + 1] as i32 - '0' as i32};
        for _ in 0..count {
            blocks.push(block_num);
        }
        for _ in 0..free_count {
            blocks.push(-1);
        }
    }

    blocks.iter().for_each(|b| print!("{} ", b));
    println!();

    let mut last_block = find_last_used(&blocks, blocks.len() as isize - 1);

    for i in 0..blocks.len() {
        if i >= last_block as usize { break }
        if blocks[i] == -1 {
            blocks[i] = blocks[last_block as usize];
            blocks[last_block as usize] = -1;
            last_block = find_last_used(&blocks, last_block - 1);
        }
    }

    blocks.iter().for_each(|b| print!("{} ", b));
    println!();

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            total += blocks[i] as i64 * i as i64;
        }
    }
    println!("{}", total);
}

fn find_last_used(blocks: &Vec<i32>, mut last_block: isize) -> isize {
    while last_block >= 0 && blocks[last_block as usize] == -1 {
        last_block -= 1;
    }
    last_block
}
