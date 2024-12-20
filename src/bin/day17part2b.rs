fn main() {
    let v = [2, 4, 1, 1, 7, 5, 1, 5, 4, 3, 5, 5, 0, 3, 3, 0];
    let a = require(&v, 0);
    println!("a: {:?}", a);
}

fn require(v: &[u64; 16], index: usize) -> Vec<u64> {
    if index == v.len() {
        vec![0]
    } else {
        let mut possiblities = Vec::new();
        for mut top in require(v, index + 1) {
            top <<= 3;
            for i in 0..8 {
                let a = top | i;
                let calc = (a >> ((a % 8) ^ 1) ^ 4 ^ (a % 8)) % 8;
                println!("checking {} {:o} against {} for index {}", calc, a, v[index], index);
                if calc == v[index] {
                    possiblities.push(a);
                }
            }
        }
        possiblities
    }
}