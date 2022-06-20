use std::io;

pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c);
    }

    result
}

fn main() {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "END" {
            break;
        } else {
            println!("{}", reverse(&s));
        }
    }
}
