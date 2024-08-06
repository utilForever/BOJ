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
    let n = input_integers()[0] as usize;

    let mut num_tiles: [i32; 1001] = [0; 1001];
    num_tiles[1] = 1;
    num_tiles[2] = 3;

    for i in 3..=n {
        num_tiles[i] = (2 * num_tiles[i - 2] + num_tiles[i - 1]) % 10007;
    }

    println!("{}", num_tiles[n]);
}
