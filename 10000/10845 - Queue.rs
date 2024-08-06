use std::{collections::VecDeque, io};

fn input_strings() -> Vec<String> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<String> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

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
    let mut queue = VecDeque::new();

    let n = input_integers()[0];

    for _ in 0..n {
        let order = input_strings();

        match order[0].as_str() {
            "push" => {
                let num = order[1].parse::<i64>().unwrap();
                queue.push_back(num);
            }
            "pop" => {
                if queue.is_empty() {
                    println!("-1");
                } else {
                    println!("{}", queue.pop_front().unwrap());
                }
            }
            "size" => {
                println!("{}", queue.len());
            }
            "empty" => {
                if queue.is_empty() {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            "front" => {
                if queue.is_empty() {
                    println!("-1");
                } else {
                    println!("{}", queue.front().unwrap());
                }
            }
            "back" => {
                if queue.is_empty() {
                    println!("-1");
                } else {
                    println!("{}", queue.back().unwrap());
                }
            }
            _ => {}
        }
    }
}
