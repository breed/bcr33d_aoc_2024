use std::cmp::max;
use bcr33d_aoc_2024::MyIn;
use maplit::hashset;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("day22 part 2");
    let mut myin = MyIn::new();
    {
        let mut groups: Vec<HashSet<String>> = Vec::new();
        let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
        loop {
            let line = myin.read_line();
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split('-').collect();
            let a = parts[0];
            let b = parts[1];
            let mut pair: HashSet<String> = HashSet::new();
            pair.insert(a.to_string());
            pair.insert(b.to_string());
            groups.push(pair);
            connections
                .entry(a.to_string())
                .or_insert(HashSet::from([a.to_string()]))
                .insert(b.to_string());
            connections
                .entry(b.to_string())
                .or_insert(HashSet::from([b.to_string()]))
                .insert(a.to_string());
        }

        loop {
            let mut changed = false;

            connections.keys().for_each(|node| {
                let cons = &connections[node];
                for mut group in &mut groups {
                    if !group.contains(node) && cons.is_superset(group) {
                        changed = true;
                        group.insert(node.to_string());
                    }
                }
            });
            if !changed {
                break;
            }
        }
        let mut max_group: &HashSet<String> = &HashSet::new();
        for group in &groups {
            if group.len() > max_group.len() {
                max_group = group;
            }
        }
        let mut answer_list: Vec<String> = max_group.iter().cloned().collect();
        answer_list.sort();
        println!("{}", answer_list.join(","));
    }
}
