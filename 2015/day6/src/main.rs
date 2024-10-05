use std::fs;
fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file.");
    let input = input.trim().lines();

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for _i in 0..1000 {
        let mut inner: Vec<bool> = Vec::new();

        for _j in 0..1000 {
            inner.push(false);
        }
        grid.push(inner);
    }

    fn parse_nums(s: &str) -> (usize, usize, usize, usize) {
        let v: Vec<&str> = s.split(" ").filter(|x| x.contains(",")).collect();
        return (
        v[0].split(",").nth(0).unwrap().parse().unwrap(),
        v[1].split(",").nth(0).unwrap().parse().unwrap(),
        v[0].split(",").nth(1).unwrap().parse().unwrap(),
        v[1].split(",").nth(1).unwrap().parse().unwrap()
        );
    }


    for i in input.clone() {
        let coord = parse_nums(i);

        for k in coord.0..=coord.1 {
            for l in coord.2..=coord.3 {
                if i.contains("turn on") {
                    grid[k][l] = true;
                } else if i.contains("turn off") {
                    grid[k][l] = false;
                } else if grid[k][l] == false {
                    grid[k][l] = true;
                } else {
                    grid[k][l] = false;
                }
            }
        }
    }

    let mut num = 0;
    for i in grid {
        for j in i {
            if j { num += 1; }
        }
    }

    println!("{}", num);

    // Part 2

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _i in 0..1000 {
        let mut inner: Vec<i32> = Vec::new();

        for _j in 0..1000 {
            inner.push(0);
        }
        grid.push(inner);
    }


    for i in input {
        let coord = parse_nums(i);

        for j in coord.0..=coord.1 {
            for k in coord.2..=coord.3 {
                if i.contains("turn on") {
                    grid[j][k] += 1;
                } else if i.contains("toggle") {
                    grid[j][k] += 2;
                } else if grid[j][k] == 0 {
                    continue;
                } else {
                    grid[j][k] -= 1;
                }
            }
        }
    }

    let mut num = 0;
    for i in grid {
        for j in i {
            num += j;
        }
    }

    println!("{}", num);

}




// answer should be 569999 after rewrite.