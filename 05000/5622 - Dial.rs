use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut ret = 0;

    for c in s.chars() {
        match c {
            'A'..='C' => ret += 3,
            'D'..='F' => ret += 4,
            'G'..='I' => ret += 5,
            'J'..='L' => ret += 6,
            'M'..='O' => ret += 7,
            'P'..='S' => ret += 8,
            'T'..='V' => ret += 9,
            'W'..='Z' => ret += 10,
            _ => unreachable!(),
        }
    }

    println!("{}", ret);
}
