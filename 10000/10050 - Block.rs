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

fn process_block(n: i32, offset: i32) {
    if n == 3 {
        println!("2 to -1");
        println!("5 to 2");
        println!("3 to -3");
    } else if n == 4 {
        println!("{} to {}", 6 + offset, -1 + offset);
        println!("{} to {}", 3 + offset, 6 + offset);
        println!("{} to {}", 0 + offset, 3 + offset);
        println!("{} to {}", 7 + offset, 0 + offset);
    } else if n == 5 {
        println!("{} to {}", 8 + offset, -1 + offset);
        println!("{} to {}", 3 + offset, 8 + offset);
        println!("{} to {}", 6 + offset, 3 + offset);
        println!("{} to {}", 0 + offset, 6 + offset);
        println!("{} to {}", 9 + offset, 0 + offset);
    } else if n == 6 {
        println!("{} to {}", 10 + offset, -1 + offset);
        println!("{} to {}", 7 + offset, 10 + offset);
        println!("{} to {}", 2 + offset, 7 + offset);
        println!("{} to {}", 6 + offset, 2 + offset);
        println!("{} to {}", 0 + offset, 6 + offset);
        println!("{} to {}", 11 + offset, 0 + offset);
    } else if n == 7 {
        println!("{} to {}", 8 + offset, -1 + offset);
        println!("{} to {}", 5 + offset, 8 + offset);
        println!("{} to {}", 12 + offset, 5 + offset);
        println!("{} to {}", 3 + offset, 12 + offset);
        println!("{} to {}", 9 + offset, 3 + offset);
        println!("{} to {}", 0 + offset, 9 + offset);
        println!("{} to {}", 13 + offset, 0 + offset);
    } else {
        println!("{} to {}", n * 2 - 2 + offset, -1 + offset);
        println!("{} to {}", 3 + offset, n * 2 - 2 + offset);

        process_block(n - 4, offset + 4);

        println!("{} to {}", 0 + offset, n * 2 - 5 + offset);
        println!("{} to {}", n * 2 - 1 + offset, 0 + offset);
    }
}

fn main() {
    let n = input_integers()[0];

    process_block(n, 0);
}
