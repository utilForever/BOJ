use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let nums = input_integers();

    if nums[0] != 1 && nums[0] != 8 {
        println!("mixed");
        return;
    }

    for i in 1..8 {
        if (nums[0] == 1 && nums[i] != i as i64 + 1) || (nums[0] == 8 && nums[i] != 8 - i as i64) {
            println!("mixed");
            return;
        }
    }

    if nums[0] == 1 {
        println!("ascending");
    } else {
        println!("descending");
    }
}
