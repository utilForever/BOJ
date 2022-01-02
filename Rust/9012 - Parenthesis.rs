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
    let t = input_integers()[0];

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        println!("{}", if is_valid(s) { "YES" } else { "NO" });
    }
}

fn is_valid(s: String) -> bool {
    let mut num_parenthesis = 0;

    for i in 0..s.chars().count() {
        if s.chars().nth(i).unwrap() == '(' {
            num_parenthesis += 1;
        } else if s.chars().nth(i).unwrap() == ')' {
            num_parenthesis -= 1;
        }

        if num_parenthesis < 0 {
            return false;
        }
    }

    if num_parenthesis == 0 {
        return true;
    }

    false
}
