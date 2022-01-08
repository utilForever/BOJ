use std::io;

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
    let mut stack = Vec::new();

    let n = input_integers()[0];

    for _ in 0..n {
        let order = input_strings();

        match order[0].as_str() {
            "push" => {
                let num = order[1].parse::<i64>().unwrap();
                stack.push(num);
            }
            "pop" => {
                if stack.is_empty() {
                    println!("-1");
                } else {
                    println!("{}", stack.pop().unwrap());
                }
            }
            "size" => {
                println!("{}", stack.len());
            }
            "empty" => {
                if stack.is_empty() {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            "top" => {
                if stack.is_empty() {
                    println!("-1");
                } else {
                    println!("{}", stack[stack.len() - 1]);
                }
            }
            _ => {}
        }
    }
}
