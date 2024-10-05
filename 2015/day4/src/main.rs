use std::ops::Add;
use md5::{compute, Digest};

fn main() {

    let mut flag = true;

    let mut num = 0;

    let input = "yzbqklnj".to_string();
    while flag {
        let hasher = input.clone().add(&num.to_string());
        let hashed = compute(hasher).to_vec();
        if hashed[0] == 0 && hashed[1] == 0 && hashed[2] < 16 {
            println!("{}", num);
            flag = false;
        }
        num += 1;
    }
    flag = true;
    while flag {
        let hasher = input.clone().add(&num.to_string());
        let hashed = compute(hasher).to_vec();
        if hashed[0] == 0 && hashed[1] == 0 && hashed[2] == 0 {
            println!("{}", num);
            flag = false;
        }
        num += 1;
    }


}
