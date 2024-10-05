use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    let input = fs::read_to_string("input").expect("Couldn't read file");
    for line in input.trim().lines() {
        let mut entry = line.split('-');
        let entry: (&str, &str) = (entry.next().unwrap(), entry.next().unwrap());
        println!("{:?}", entry);
        if !map.contains_key(entry.0) {
            map.insert(entry.0, HashSet::new());
        }
        map.get(entry.0).unwrap().insert( entry.1);
        if !map.contains_key(entry.1) {
            map.insert(entry.1, HashSet::new());
        }
        map.get(entry.1).unwrap().insert(entry.0);
    }



    for i in map {
        for j in i.1 {
            println!("{} - {}", i.0, j);
        }
    }
}
