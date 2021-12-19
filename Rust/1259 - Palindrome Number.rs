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
    loop {
        let mut n = input_integers()[0];

        if n == 0 {
            break;
        }

        let mut digit = 0;
        let mut num = [0; 5];

        while n >= 10 {
            num[digit] = n % 10;
            n /= 10;
            digit += 1;
        }

        num[digit] = n;
        digit += 1;

        let mut is_palindrome = true;

        for i in 0..digit / 2 {
            if num[i] != num[digit - i - 1] {
                is_palindrome = false;
                break;
            }
        }

        println!("{}", if is_palindrome { "yes" } else { "no" });
    }
}
