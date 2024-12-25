use crate::Definition::{Expression, Literal};
use crate::Operator::{And, Or, Xor};
use bcr33d_aoc_2024::MyIn;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
enum Definition {
    Literal(u8),
    Expression(String, Operator, String),
}
fn main() {
    println!("day24 part 2");
    let mut myin = MyIn::new();
    let mut definitions = HashMap::new();

    let mut highest_xbit:u64 = 0;
    let mut x: u64 = 0;
    let mut highest_ybit:u64 = 0;
    let mut y: u64 = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let a = parts.next().unwrap().to_string();
        let b = if parts.next().unwrap() == "1" {
            1 as u8
        } else {
            0 as u8
        };
        let bit = a[1..].parse::<u64>().unwrap();
        if a.starts_with("x") {
            x |= (b as u64) << bit;
            if bit > highest_xbit {
                highest_xbit = bit;
            }
        } else if a.starts_with("y") {
            y |= (b as u64) << bit;
            if bit > highest_ybit {
                highest_ybit = bit;
            }
        }
        definitions.insert(a, Literal(b));
    }

    let mut highest_zbit: u64 = 0;
    loop {
        let line = myin.read_line();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" -> ");
        let exp = parts.next().unwrap().to_string();
        let result = parts.next().unwrap().to_string();
        if result.starts_with("z") {
            let bit = result[1..].parse::<u64>().unwrap();
            if bit > highest_zbit {
                highest_zbit = bit;
            }
        }
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

    // set all xs and ys to 0
    for i in 0..highest_xbit + 1 {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        definitions.insert(x, Literal(0));
        definitions.insert(y, Literal(0));
    }

    let mut bad_counts = HashMap::new();
    let mut good_nodes = HashMap::new();

    find_problem_z(&definitions, highest_zbit, 0, &mut bad_counts,  &mut good_nodes, "000");

    // now probe each bit
    for i in 0..highest_xbit {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        let xx = format!("x{:02}", i+1);
        let yy = format!("y{:02}", i+1);
        *definitions.get_mut(&x).unwrap() = Literal(1);
        find_problem_z(&definitions, highest_zbit,1 << i, &mut bad_counts, &mut good_nodes, format!("{}x=1", i).as_str());
        *definitions.get_mut(&y).unwrap() =  Literal(1);
        find_problem_z(&definitions, highest_zbit, 0b10 << i, &mut bad_counts, &mut good_nodes, format!("{}xy3", i).as_str());
        *definitions.get_mut(&xx).unwrap() = Literal(1);
        find_problem_z(&definitions, highest_zbit,(0b100 << i), &mut bad_counts, &mut good_nodes, format!("{}x=1", i).as_str());
        *definitions.get_mut(&yy).unwrap() =  Literal(1);
        find_problem_z(&definitions, highest_zbit, (0b110 << i), &mut bad_counts, &mut good_nodes, format!("{}xy3", i).as_str());
        *definitions.get_mut(&xx).unwrap() =  Literal(0);
        *definitions.get_mut(&yy).unwrap() =  Literal(0);
        *definitions.get_mut(&x).unwrap() =  Literal(0);
        find_problem_z(&definitions, highest_zbit,1 << i, &mut bad_counts, &mut good_nodes, format!("{}y=1", i).as_str());
        *definitions.get_mut(&y).unwrap() = Literal(0);
    }

    let mut counts: Vec<(&String, &i32)> = bad_counts.iter().collect();
    counts.sort_by(|a, b| a.1.cmp(b.1));
    counts.reverse();
    counts.iter().take(20).for_each(|(k, v)| {
        //println!("{} {}", k, v);
    });
    //println!("------");
    let mut counts: Vec<(&String, &i32)> = bad_counts.iter().filter(|(k,v)| k.starts_with("z")).collect();
    counts.sort_by(|a, b| a.1.cmp(b.1));
    counts.reverse();
    counts.iter().take(20).for_each(|(k, v)| {
      //  println!("{} {}", k, v);
    });
}

fn find_problem_z(definitions: &HashMap<String, Definition>, highest_bit: u64, expected: u64, bad_counts: &mut HashMap<String, i32>, good_nodes: &mut HashMap<String, i32>, prefix: &str) {
    let z = calc_z(&definitions, highest_bit, expected, bad_counts, good_nodes);
}

fn calc_z(definitions: &HashMap<String, Definition>, highest_bit: u64, expected: u64, bad_counts: &mut HashMap<String, i32>, good_nodes: &mut HashMap<String, i32>) {
    println!("checking {:b}", expected);
    for i in 0..highest_bit + 1 {
        let mut involved = HashMap::new();
        let mask = 1 << i;
        let ebit = if expected & mask != 0 {1} else {0};
        let zbit = resolve(&format!("z{:02}", i), ebit, &definitions, &mut involved) as u8;
        if zbit != ebit {
            //println!(" z{:02}:{}>{}", i, zbit, ebit);
            involved.iter().filter(|(k,_)| !k.starts_with("x") && !k.starts_with("y")).for_each(|(k, c)| {
                let count = bad_counts.entry(k.to_string()).or_insert(0);
                *count += c;
            });
            *bad_counts.entry(format!("z{:02}", i)).or_insert(0) += 1;
        } else {
            involved.iter().filter(|(k,_)| !k.starts_with("x") && !k.starts_with("y")).for_each(|(k, c)| {
                let count = good_nodes.entry(k.to_string()).or_insert(0);
                *count += c;
            });
            *good_nodes.entry(format!("z{:02}", i)).or_insert(0) += 1;

        }
    }
}
fn resolve(key: &String, ebit: u8, definitions: &HashMap<String, Definition>, involved: &mut HashMap<String, i32>) -> u8 {
    // println!("resolving {} {}", key, ebit);
    match &definitions[key] {
        Literal(v) => {
            *v
        },
        Expression(a, op, b) => {
            *involved.entry(a.to_string()).or_insert(0) += 1;
            *involved.entry(b.to_string()).or_insert(0) += 1;
            match op {
                And => {
                    if ebit == 1 {
                        let ar = resolve(a, 1, definitions, involved);
                        let br = resolve(b, 1, definitions, involved);
                        if (ar != 1) {
                            println!("BAD AND CONNECT {} -> {} expected {} got {}", key, a, ebit, ar);
                        }
                        if (br != 1) {
                            println!("BAD AND CONNECT {} -> {} expected {} got {}", key, b, ebit, br);
                        }
                        ar & br
                    } else {
                        let ar = resolve(a, 3, definitions, involved);
                        let br = resolve(b, 3, definitions, involved);
                        let r = ar & br;
                        if ebit == 0 && r != 0 {
                            println!("BAD AND CONNECT {} either {} {}", key, a, b);
                        }
                        r
                    }
                },
                Or => {
                    if ebit == 0 {
                        let ar = resolve(a, 0, definitions, involved);
                        let br = resolve(b, 0, definitions, involved);
                        if (ar != 0) {
                            println!("BAD OR CONNECT {} -> {} {}", key, a, ebit);
                        }
                        if (br != 0) {
                            println!("BAD OR CONNECT {} -> {} {}", key, b, ebit);
                        }
                        ar | br
                    } else {
                        let ar = resolve(a, 3, definitions, involved);
                        let br = resolve(b, 3, definitions, involved);
                        let r = ar | br;
                        if ebit == 1 && r != 1 {
                            println!("BAD OR CONNECT {} either {}({}) {}({})", key, a, ar, b, br);
                        }
                        r
                    }
                },
                Xor => {
                    let ar = resolve(a, 3, definitions, involved);
                    let br = resolve(b, 3, definitions, involved);
                    let r = ar ^ br;
                    if ebit != 3 && r != ebit {
                        //println!("BAD XOR CONNECT {} either {}({}) {}({}) {} != {}", key, a, ar, b, br, r, ebit);
                       /*
                        println!(">>>>>>Trying 1");
                        let ar = resolve(a, 1, definitions, involved);
                        let br = resolve(b, ebit^1, definitions, involved);
                        let r = ar ^ br;
                        if r != ebit {
                            println!("TRY BAD XOR CONNECT {} either {}({}) {}({}) {} != {}", key, a, ar, b, br, r, ebit);
                            let ar = resolve(a, 0, definitions, involved);
                            let br = resolve(b, ebit^0, definitions, involved);
                            let r = ar ^ br;
                            if r != ebit {
                                println!("TRYBAD XOR CONNECT {} either {}({}) {}({}) {} != {}", key, a, ar, b, br, r, ebit);
                            }
                        }
                        println!(">>>>> DONE trying");

                        */
                    }
                    r
                },
            }
        },
    }
}
