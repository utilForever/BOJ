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
    let _ = input_integers()[0] as usize;

    let mut sum = 0_i64;
    let mut multiplier = 1_i64;

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    for c in s.chars() {
        sum += ((c as u8 - 'a' as u8 + 1) as i64 * multiplier) % 1_234_567_891;
        multiplier *= 31;
        multiplier %= 1_234_567_891;
    }

    println!("{}", sum % 1_234_567_891);
}
