use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");

    let input: Vec<i32> = input
        .trim()
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .collect();

    // Part 1
    let part_one: i32 = input.iter().sum::<i32>();
    println!("{:?}", part_one);

    // Part two
    let mut part_two = 0;
    let mut sum = 0;

    for i in input {
        sum += i;
        part_two += 1;
        if sum == -1 {
            break;
        }
    }
    println!("{}", part_two);
}
