use bcr33d_aoc_2024::MyIn;
use maplit::hashmap;
use std::collections::HashMap;

fn main() {
    let keypad_coords: HashMap<char, (i32, i32)> = hashmap! {
    '1' => (2, 0),
    '2' => (2, 1),
    '3' => (2, 2),
    '4' => (1, 0),
    '5' => (1, 1),
    '6' => (1, 2),
    '7' => (0, 0),
    '8' => (0, 1),
    '9' => (0, 2),
    '0' => (3, 1),
    'A' => (3, 2),
    'B' => (3, 0)
};

    let dir_coords: HashMap<char, (i32, i32)> = hashmap! {
    'B' => (0,0),
    '^' => (0,1),
    'A' => (0,2),
    '<' => (1,0),
    'v' => (1,2),
    '>' => (2,0)
};

    println!("day21 part 1");
    let mut myin = MyIn::new();

    {
        loop {
            let line = myin.read_line();
            if line.is_empty() {
                break;
            }
            let mut row = 3;
            let mut col = 2;
            let code_num = line[..3].parse::<i32>().unwrap();
            let mut keypad_presses = Vec::new();
            for c in line.chars() {
                let (dr, dc) = keypad_coords[&c];
                if row == 3 && dc == 0 {
                    move_horizontal(&mut col, &mut keypad_presses, dc);
                    move_up(&mut row, &mut keypad_presses, dr);
                } else {
                    move_down(&mut row, &mut keypad_presses, dr);
                    move_up(&mut row, &mut keypad_presses, dr);
                    move_horizontal(&mut col, &mut keypad_presses, dc);
                }
                keypad_presses.push('A');
            }
            for _ in 0..3 {
                row = 0;
                col = 2;
                println!("code_num: {} {:?} {}", code_num, keypad_presses.iter().collect::<String>(), keypad_presses.len());
                let mut new_keypad_presses = Vec::new();
                for c in &keypad_presses {
                    let (dr, dc) = dir_coords[&c];
                    if row == 0 && dc == 0 {
                        move_horizontal(&mut col, &mut new_keypad_presses, dc);
                        move_down(&mut row, &mut new_keypad_presses, dr);
                    } else {
                        move_up(&mut row, &mut new_keypad_presses, dr);
                        move_down(&mut row, &mut new_keypad_presses, dr);
                        move_horizontal(&mut col, &mut new_keypad_presses, dc);
                    }
                    new_keypad_presses.push('A');
                }
                keypad_presses = new_keypad_presses;
            }
            println!("code_num: {} {:?} {}", code_num, keypad_presses.iter().collect::<String>(), keypad_presses.len());
        }
    }
}

fn keypad_cost(row: i32, col: i32, code: Vec<u8>, index: usize) -> i32 {
    if index == code.len() {
        return 0;
    }
    if row == 3 && dc == 0 {
        move_horizontal(&mut col, &mut keypad_presses, dc);
        move_up(&mut row, &mut keypad_presses, dr);
    } else {
        move_down(&mut row, &mut keypad_presses, dr);
        move_up(&mut row, &mut keypad_presses, dr);
        move_horizontal(&mut col, &mut keypad_presses, dc);
    }

}

fn move_down(row: &mut i32, keypad_presses: &mut Vec<char>, dr: i32) {
    while dr > *row {
        keypad_presses.push('v');
        *row += 1;
    }
}

fn move_up(row: &mut i32, keypad_presses: &mut Vec<char>, dr: i32) {
    while dr < *row {
        keypad_presses.push('^');
        *row -= 1;
    }
}

fn move_horizontal(col: &mut i32, keypad_presses: &mut Vec<char>, dc: i32) {
    while dc < *col {
        keypad_presses.push('<');
        *col -= 1;
    }
    while dc > *col {
        keypad_presses.push('>');
        *col += 1;
    }
}
