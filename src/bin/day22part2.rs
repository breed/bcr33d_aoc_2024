use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day21 part 2");
    let mut myin = MyIn::new();

    {
        let mut changes: Vec<[i8; 2000]> = Vec::new();
        let mut prices: Vec<[i8; 2000]> = Vec::new();

        loop {
            let line = myin.read_line();
            if line.is_empty() {
                break;
            }
            let mut pa = [0 as i8; 2000];
            let mut ca = [0 as i8; 2000];
            let mut secret = line.parse::<u64>().unwrap();
            for i in 0..2000 {
                secret ^= secret * 64; // Calculate the result of multiplying the secret number by 64.Then, mix this result into the secret number.
                secret %= 16777216; // Finally, prune the secret number.
                let result = secret / 32; //  Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer.
                secret ^= result; // Then, mix this result into the secret number.
                secret %= 16777216; // Finally, prune the secret number.
                let result2 = secret * 2048; // Calculate the result of multiplying the secret number by 2048.
                secret ^= result2; // Then, mix this result into the secret number.
                secret %= 16777216; // Finally, prune the secret
                pa[i] = (secret % 10) as i8;
                if i != 0 {
                    ca[i] = pa[i] - pa[i - 1];
                } else {
                    ca[i] = 66;
                }
            }
            println!("{:?}", ca);
            changes.push(ca);
            prices.push(pa);
        }

        let mut totals = [[[[-1; 20];20];20];20];
        let mut max_total = 0;
        for d0 in -9..9 {
            for d1 in -9..9 {
                for d2 in -9..9 {
                    for d3 in -9..9 {
                        let mut total = 0 as u64;
                        for i in 0..changes.len() {
                            let ca = &changes[i];
                            for index in 1..ca.len()-4 {
                                if ca[index] != d0 {
                                    continue;
                                }
                                if ca[index + 1] != d1 {
                                    continue;
                                }
                                if ca[index + 2] != d2 {
                                    continue;
                                }
                                if ca[index+3] == d3 {
                                    total += prices[i][index + 3] as u64;
                                    break;
                                }
                            }
                        }
                        if total > max_total {
                            println!("[{} {} {} {}] {}", d0, d1, d2, d3, total);
                            max_total = total;
                        }
                    }
                }
            }
        }
        println!("total {}", max_total);
    }
}
