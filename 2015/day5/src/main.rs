use std::collections::HashSet;
use std::fs;
fn main() {
    let input: String = fs::read_to_string("input").expect("Couldn't read file.");
    let input: Vec<&str> = input.trim().lines().collect();

    println!("{}", input.len());
    let mut v_one: Vec<&str> = Vec::new();

    // Test for at least 3 vowels in entry.

    let mut num: i32 = 0;
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    for i in &input {
        for c in i.chars() {
            if vowels.contains(&c) {
                num += 1;
            }
        }
        if num >= 3 {
            v_one.push(i);
        }
        num = 0;
    }

    println!("{}", v_one.len());

    let mut v_two: Vec<&str> = Vec::new();

    // Test for double letter.

    for i in v_one {
        let mut current = ' ';
        let mut flag = false;
        for c in i.chars() {
            if c == current {
                flag = true;
            }
            current = c;
        }
        if flag { v_two.push(i); }
    }

    println!("{}", v_two.len());

    let mut v_three: Vec<&str> = Vec::new();
    //Testing for forbidden patterns.

    let forbidden = ["ab", "cd", "pq", "xy"];
    for i in v_two {
        if !(i.contains(forbidden[0])
            || i.contains(forbidden[1])
            || i.contains(forbidden[2])
            || i.contains(forbidden[3])) {
            v_three.push(i);
        }
    }

    println!("{}", v_three.len());

    // Part two-

    println!("{}", input.len());
    let mut vec_one = Vec::new();

    let mut flag = false;
    for i in &input {
        for j in 2..i.len() {
            if i.chars().nth(j).unwrap() == i.chars().nth(j - 2).unwrap() {
                flag = true
            }

        }
        if flag { vec_one.push(i) }
        flag = false;
    }

    println!("{}", vec_one.len());

    let mut vec_two = Vec::new();

    let mut flag = false;
    for i in vec_one {
        for j in 0..i.len()-3 {
            if i[j+2..i.len()].contains(&i[j..=j+1]) {
                flag = true;
            }
        }
        if flag { vec_two.push(i); }
        flag = false;
    }

    println!("{:?}", vec_two.len());

}