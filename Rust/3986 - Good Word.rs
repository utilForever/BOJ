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
    let mut ans = 0;

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        if is_valid(s) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c != 'A' && c != 'B' {
            continue;
        }

        if stack.is_empty() {
            stack.push(c);
        } else {
            if *stack.last().unwrap() == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }

    stack.is_empty()
}
