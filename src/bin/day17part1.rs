use std::ptr::read;
use bcr33d_aoc_2024::MyIn;

struct Registers {
    A: i32,
    B: i32,
    C: i32,
}

fn main() {
    let mut myin = MyIn::new();
    let mut regs = Registers{A: read_reg(&mut myin), B: read_reg(&mut myin), C: read_reg(&mut myin)};

    // skip Program:
    myin.read_word();
    myin.read_word();

    let prog = myin.read_word().split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let mut ip = 0;
    loop {
        let opcode = prog[ip];
        let operand = prog[ip + 1];
        ip += 2;
        match opcode {
            0 => regs.A /= 1 << get_combo(&regs, operand),
            1 => regs.B ^= operand,
            2 => regs.B = get_combo(&regs, operand) % 8,
            3 => ip = if regs.A == 0 { ip} else { operand as usize},
            4 => regs.B ^= regs.C,
            5 => print!("{},", get_combo(&regs, operand) % 8),
            6 => regs.B = regs.A / (1 << get_combo(&regs, operand)),
            7 => regs.C = regs.A / (1 << get_combo(&regs, operand)),
            _ => panic!("bad opcode {}", opcode),
        }
        if ip >= prog.len() {
            break;
        }
    }
    println!("\nA: {}, B: {}, C: {}", regs.A, regs.B, regs.C);
}

fn get_combo(regs: &Registers, operand: i32) -> i32 {
    match operand {
        0..4 => operand as i32,
        4 => regs.A,
        5 => regs.B,
        6 => regs.C,
        _ => panic!("bad operand"),
    }
}

fn read_reg(myin: &mut MyIn) -> i32 {
    myin.read_word();
    myin.read_word();
    myin.read_word().parse().unwrap()
}