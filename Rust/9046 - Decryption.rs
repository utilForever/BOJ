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

        let chars = s.chars().collect::<Vec<_>>();
        let mut alphabets = vec![0; 26];

        for c in chars {
            if !c.is_alphabetic() {
                continue;
            }

            alphabets[(c as u8 - 'a' as u8) as usize] += 1;
        }

        let ret = *alphabets.iter().max().unwrap();

        writeln!(
            out,
            "{}",
            if alphabets.iter().filter(|&x| *x == ret).count() == 1 {
                (alphabets.iter().position(|&x| x == ret).unwrap() as u8 + 'a' as u8) as char
            } else {
                '?'
            }
        )
        .unwrap();
    }
}
