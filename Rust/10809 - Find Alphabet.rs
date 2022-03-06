use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut alphabet = [-1; 26];

    let mut chars = s.chars();
    for i in 0..s.len() {
        let c = chars.next().unwrap();
        let idx = (c.to_ascii_lowercase() as u8 - b'a') as usize;

        if alphabet[idx] == -1 {
            alphabet[idx] = i as i32;
        }
    }

    for ch in alphabet.iter() {
        print!("{} ", ch);
    }

    println!("");
}
