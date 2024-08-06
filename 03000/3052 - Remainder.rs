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
    let mut nums = vec![0; 42];
    let mut result = 0;

    for _ in 0..10 {
        let num = input_integers()[0] as usize;

        if nums[num % 42] == 0 {
            nums[num % 42] = 1;
            result += 1;
        }
    }

    println!("{}", result);
}
