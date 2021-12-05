use std::io;

fn get_num_palindrome(sequence: &mut Vec<Vec<i32>>, s: &str, a: i32, b: i32) -> i32 {
    if a > b {
        return 0;
    }

    if sequence[a as usize][b as usize] != -1 {
        return sequence[a as usize][b as usize];
    }

    sequence[a as usize][b as usize] = 0;

    for i in a..b {
        if s.as_bytes()[i as usize] == s.as_bytes()[b as usize] {
            sequence[a as usize][b as usize] += get_num_palindrome(sequence, s, i + 1, b - 1) + 1;
        }
    }

    sequence[a as usize][b as usize] += get_num_palindrome(sequence, s, a, b - 1) + 1;

    sequence[a as usize][b as usize]
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    // Ignore '\r' and '\n'
    s = s.trim().to_string();

    let mut sequence = vec![vec![-1; 30]; 30];

    for i in 0..s.len() as usize {
        sequence[i][i] = 1;
    }

    println!(
        "{}",
        get_num_palindrome(&mut sequence, &s, 0, s.len() as i32 - 1)
    );
}
