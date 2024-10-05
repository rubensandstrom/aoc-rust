
use std::fs;
fn main() {

	let input = fs::read_to_string("input")
        .expect("Couldn't read file");
	let input: Vec<&str> = input.lines().collect();
	let mut input_vec: Vec<Vec<i32>> = vec!();
	for lines in &input {
		input_vec.push(lines.split("x").map(|x| x.parse().unwrap_or(0) ).collect());
	}


	let mut sum_one = 0;
	let mut sum_two = 0;

	fn box_area(vec: Vec<i32>) -> i32 {
		let mut rv = 0;

		rv += 2*vec[0]*vec[1];
		rv += 2*vec[0]*vec[2];
		rv += 2*vec[1]*vec[2];

		if vec[0]*vec[1] <= vec[0]*vec[2] && vec[0]*vec[1] <= vec[1]*vec[2] { rv += vec[0]*vec[1]; }
		else if vec[1]*vec[2] <= vec[0]*vec[2] && vec[1]*vec[2] <= vec[0]*vec[1] { rv += vec[1]*vec[2]; }
		else if vec[0]*vec[2] <= vec[0]*vec[1] && vec[0]*vec[2] <= vec[1]*vec[2] { rv += vec[0]*vec[2]; }

		return rv
	}

	fn rib_len(vec: Vec<i32>) -> i32 {
		let mut rv = 0;

		rv += vec[0]*vec[1]*vec[2];

		if vec[0]+vec[1] <= vec[0]+vec[2] && vec[0]+vec[1] <= vec[1]+vec[2] { rv += 2*(vec[0]+vec[1]); }
		else if vec[1]+vec[2] <= vec[0]+vec[2] && vec[1]+vec[2] <= vec[0]+vec[1] { rv += 2*(vec[1]+vec[2]); }
		else if vec[0]+vec[2] <= vec[0]+vec[1] && vec[0]+vec[2] <= vec[1]+vec[2] { rv += 2*(vec[0]+vec[2]); }

		return rv
	}



	for i in &input_vec {
		sum_one += box_area(i.to_vec());
	}

	for i in &input_vec {
		sum_two += rib_len(i.to_vec());
	}
	println!("paper area: {} _ ribbon length: {}", sum_one, sum_two);


}
