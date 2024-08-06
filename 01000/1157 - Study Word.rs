use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut alphabet = [0; 26];

    let mut chars = s.chars();
    for _ in 0..s.len() {
        let c = chars.next().unwrap();
        alphabet[(c.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
    }

    let mut max_count = 0;
    let mut max_count_alphabet = 0;
    let mut is_same_count = false;

    for i in 0..26 {
        if alphabet[i] == max_count {
            is_same_count = true;
        }

        if alphabet[i] > max_count {
            max_count = alphabet[i];
            max_count_alphabet = i;
            is_same_count = false;
        }
    }

    if is_same_count {
        println!("?");
    } else {
        let c = (max_count_alphabet as u8 + b'A') as char;
        println!("{}", c);
    }
}
