use std::ptr::read;
use bcr33d_aoc_2024::MyIn;

#[derive(Clone, Copy)]
struct Registers {
    A: i64,
    B: i64,
    C: i64,
}

fn main() {
    let mut myin = MyIn::new();
    let regs_initial = Registers{A: read_reg(&mut myin), B: read_reg(&mut myin), C: read_reg(&mut myin)};
    let mut trying_a = 03267275;
    let initial_a = 010000000;;
    // skip Program:
    myin.read_word();
    myin.read_word();

    let prog = myin.read_word().split(",").map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    let mut ip ;
    let mut max_index = 0;
    loop {
        let mut regs = regs_initial;
        regs.A = trying_a;
        ip = 0;
        let mut output = Vec::new();
        'inner:
        loop {
            let opcode = prog[ip];
            let operand = prog[ip + 1];
            ip += 2;
            match opcode {
                0 => regs.A = regs.A >> get_combo(&regs, operand),
                1 => regs.B ^= operand as i64,
                2 => regs.B = get_combo(&regs, operand) % 8,
                3 => ip = if regs.A == 0 { ip } else { operand as usize },
                4 => regs.B ^= regs.C,
                5 => {
                    let new_out = get_combo(&regs, operand) % 8;
                    let index = output.len();
                    if new_out != prog[index] as i64 {
                        if index > max_index {
                            max_index = index;
                            println!("**** {:o} looking for {} at index {} found {}", trying_a, prog[index], index, new_out);
                        }
                        break 'inner
                    }
                    output.push(new_out)
                },
                6 => regs.B = regs.A >> get_combo(&regs, operand),
                7 => regs.C = regs.A >> get_combo(&regs, operand),
                _ => panic!("bad opcode {}", opcode),
            }
            if ip >= prog.len() {
                break;
            }
        }
        if output.eq(&prog) {
            break;
        }
        trying_a += initial_a;
    }
}

fn get_combo(regs: &Registers, operand: i64) -> i64 {
    match operand {
        0..4 => operand ,
        4 => regs.A,
        5 => regs.B,
        6 => regs.C,
        _ => panic!("bad operand"),
    }
}

fn read_reg(myin: &mut MyIn) -> i64 {
    myin.read_word();
    myin.read_word();
    myin.read_word().parse().unwrap()
}