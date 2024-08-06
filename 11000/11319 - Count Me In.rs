use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let mut cnt_consonants = 0;
        let mut cnt_vowels = 0;

        for c in s.chars() {
            if c == ' ' {
                continue;
            }

            match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => cnt_vowels += 1,
                _ => cnt_consonants += 1,
            }
        }

        writeln!(out, "{cnt_consonants} {cnt_vowels}").unwrap();
    }
}
