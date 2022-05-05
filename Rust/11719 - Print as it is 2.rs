use std::io::{self, BufRead};

fn main() {
    let mut s = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        handle.read_line(&mut s).unwrap();

        if !s.is_empty() {
            print!("{s}");
            s.clear();
        } else {
            println!();
            break;
        }
    }
}
