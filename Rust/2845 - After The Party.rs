use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let nums = input_integers();
    let (l, p) = (nums[0], nums[1]);
    let total_people = l * p;

    let participants = input_integers();

    for participant in participants.iter() {
        print!("{} ", participant - total_people);
    }

    println!("");
}
