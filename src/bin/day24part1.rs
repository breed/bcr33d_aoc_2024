use std::collections::{HashMap};
use bcr33d_aoc_2024::MyIn;
use crate::Definition::{Expression, Literal};
use crate::Operator::{And, Or, Xor};

enum Operator {
    And,
    Or,
    Xor,
}

enum Definition {
    Literal(u8),
    Expression(String, Operator, String),
}
fn main() {
    println!("day24 part 1");
    let mut myin = MyIn::new();
    let mut definitions = HashMap::new();

        loop {
            let line = myin.read_line();
            if line.is_empty() {
                break;
            }
            let mut parts = line.split(": ");
            let a = parts.next().unwrap().to_string();
            let b = if parts.next().unwrap() == "1" { 1 as u8} else { 0 as u8};
            definitions.insert(a, Literal(b));
        }

    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" -> ");
        let exp = parts.next().unwrap().to_string();
        let result = parts.next().unwrap().to_string();
        let mut exp_parts = exp.split(" ");
        let a = exp_parts.next().unwrap().to_string();
        let op = match exp_parts.next().unwrap() {
            "AND" => And,
            "XOR" => Xor,
            "OR" => Or,
            _ => panic!("unexpected operator"),
        };
        let b = exp_parts.next().unwrap().to_string();
        definitions.insert(result, Expression(a, op, b));
    }

    let mut z_keys: Vec<&String> = definitions.keys().filter(|k| k.starts_with("z")).collect();
    z_keys.sort();
    z_keys.reverse();
    let mut z = 0 as u64;
    for key in z_keys {
        z <<= 1;
        z |= resolve(key, &definitions) as u64;
    }
    println!("z {}", z);
}

fn resolve(key: &String, definitions: &HashMap<String, Definition>) -> u8 {
    return match &definitions[key] {
        Literal(v) => *v,
        Expression(a, op, b) => {
            match op {
                And => resolve(a, definitions) & resolve(b, definitions),
                Or => resolve(a, definitions) | resolve(b, definitions),
                Xor => resolve(a, definitions) ^ resolve(b, definitions),
            }
        }
    }
}

