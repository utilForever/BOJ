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
    let (n, m) = (nums[0] as usize, nums[1] as usize);
    let mut fish = vec![vec!['0'; m]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..m {
            fish[i][j] = chars.next().unwrap();
        }
    }

    for i in 0..n {
        for j in (0..m).rev() {
            print!("{}", fish[i][j]);
        }

        println!();
    }
}
