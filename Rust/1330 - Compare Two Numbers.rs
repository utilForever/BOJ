use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if values[0] > values[1] {
        println!(">");
    } else if values[0] < values[1] {
        println!("<");
    } else {
        println!("==");
    }
}
