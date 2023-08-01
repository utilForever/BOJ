use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for i in 1..=n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let mut t = String::new();
        io::stdin().read_line(&mut t).unwrap();
        t = t.trim().to_string();

        let mut alphabet_s = [0_i64; 26];
        let mut alphabet_t = [0_i64; 26];

        for c in s.chars() {
            alphabet_s[(c as u8 - 'a' as u8) as usize] += 1;
        }

        for c in t.chars() {
            alphabet_t[(c as u8 - 'a' as u8) as usize] += 1;
        }

        let mut ret = 0;

        for i in 0..26 {
            ret += (alphabet_s[i] - alphabet_t[i]).abs();
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
