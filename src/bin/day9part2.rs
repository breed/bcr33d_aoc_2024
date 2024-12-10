use bcr33d_aoc_2024::MyIn;
use std::collections::HashSet;

fn main() {
    println!("day9 part 2");
    let mut myin = MyIn::new();
    let mut total: i64 = 0;

    let line = myin.read_line();
    let map: Vec<char> = line.chars().collect();
    let mut blocks = Vec::new();
    for i in (0..map.len()).step_by(2) {
        let block_num = (i / 2) as i32;
        let count = map[i] as i32 - '0' as i32;
        let free_count = if map.len() == i + 1 {
            0
        } else {
            map[i + 1] as i32 - '0' as i32
        };
        for _ in 0..count {
            blocks.push(block_num);
        }
        for _ in 0..free_count {
            blocks.push(-1);
        }
    }

    let (mut blocknum, mut move_block, mut block_len) =
        find_prev_seg(&blocks, blocks.len() as isize - 1);
    while move_block > 0 {
        let (mut next_free, mut free_len) = find_next_free(&blocks, 0);
        while move_block > next_free {
            if free_len >= block_len {
                for i in 0..block_len {
                    blocks[(next_free + i) as usize] = blocknum;
                    blocks[(move_block + i) as usize] = -1;
                }
                break;
            }
            (next_free, free_len) = find_next_free(&blocks, next_free + free_len);
        }
    }
    loop {
        let nextblocknum;
        (nextblocknum, move_block, block_len) = find_prev_seg(&blocks, move_block - 1);
        if nextblocknum == -1 {
            break;
        }
        if nextblocknum < blocknum {
            if nextblocknum != blocknum - 1 {
                println!("{} too low {}", nextblocknum, blocknum);
            }
            blocknum = nextblocknum;
            break;
        }
    }

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            total += blocks[i] as i64 * i as i64;
        }
    }

    println!("{}", total);
}

fn find_next_free(blocks: &Vec<i32>, mut start_block: isize) -> (isize, isize) {
    while start_block < blocks.len() as isize && blocks[start_block as usize] != -1 {
        start_block += 1;
    }
    let mut len = 0;
    let mut curr_block = start_block;
    while curr_block < blocks.len() as isize && blocks[curr_block as usize] == -1 {
        curr_block += 1;
        len += 1;
    }
    (start_block, len)
}

fn find_prev_seg(blocks: &Vec<i32>, mut last_block: isize) -> (i32, isize, isize) {
    while last_block >= 0 && blocks[last_block as usize] == -1 {
        last_block -= 1;
    }
    if last_block < 0 {
        return (-1, last_block, 0);
    }

    // found a block, now lets find where it starts..
    let block_num = blocks[last_block as usize];
    let mut len = 1;
    while last_block > 0 && blocks[(last_block - 1) as usize] == block_num {
        last_block -= 1;
        len += 1;
    }
    (block_num, last_block, len)
}
