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
    let mut max_value = 0;
    let mut index = 0;

    for i in 1..=9 {
        let num = input_integers()[0];

        if num > max_value {
            max_value = num;
            index = i;
        }
    }

    println!("{}", max_value);
    println!("{}", index);
}
