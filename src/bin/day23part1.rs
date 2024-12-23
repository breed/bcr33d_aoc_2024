use std::collections::{HashMap, HashSet};
use maplit::hashset;
use bcr33d_aoc_2024::MyIn;

fn main() {
    println!("day22 part 1");
    let mut myin = MyIn::new();

    {
        let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

        loop {
            let line = myin.read_line();
            if line.is_empty() {
                break;
            }
            let mut parts = line.split("-");
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            connections.entry(a.to_string()).or_insert(HashSet::new()).insert(b.to_string());
            connections.entry(b.to_string()).or_insert(HashSet::new()).insert(a.to_string());
        }

        let mut combos = HashSet::new();
        connections.iter()
            .filter(|(k, v)| k.starts_with("t") && v.len() >= 2)
            .for_each(|(k, v)| {
                let mut peers: Vec<&String> = v.iter().collect();
                peers.sort();
                for i in 0..peers.len() {
                    for j in i+1..peers.len() {
                        if connections[peers[i]].contains(peers[j]) {
                            let mut combo = vec![k.to_string(), peers[i].to_string(), peers[j].to_string()];
                            combo.sort();

                            let c = combo.join(",");
                            combos.insert(c);
                        }
                    }
                }
        });

        combos.iter().for_each(|c| println!("{}", c));
        println!("total {}", combos.len());
    }
}
