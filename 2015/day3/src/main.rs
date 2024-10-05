use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file.");

    /* Part 1 */

    let mut vec: Vec<(i32, i32)> = vec![(0, 0)];
    let mut current = (0, 0);
    for i in input.trim().chars() {
        match i {
            '<' => current.0 -= 1,
            '>' => current.0 += 1,
            'v' => current.1 -= 1,
            '^' => current.1 += 1,
            _ => panic!("FU!")
        }

        if !vec.contains(&current) {
            vec.push(current.clone())
        }
    }

    println!("{}", vec.len());

    /* Part 2 */

    let mut vec: Vec<(i32, i32)> = vec![(0, 0)];

    let mut current_one = (0, 0);
    let mut current_two = (0, 0);

    let mut x = 0;

    for i in input.trim().chars() {
        x += 1;
        if x % 2 == 0 {
            match i {
                '<' => current_two.0 -= 1,
                '>' => current_two.0 += 1,
                'v' => current_two.1 -= 1,
                '^' => current_two.1 += 1,
                _ => panic!("FU!")
            }

            if !vec.contains(&current_two) {
                vec.push(current_two.clone())
            }
            continue
        }
        match i {
            '<' => current_one.0 -= 1,
            '>' => current_one.0 += 1,
            'v' => current_one.1 -= 1,
            '^' => current_one.1 += 1,
            _ => panic!("FU!")
        }

        if !vec.contains(&current_one) {
            vec.push(current_one.clone())
        }
    }

    println!("{}", vec.len());
}

