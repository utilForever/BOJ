use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.chars().collect::<Vec<_>>();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    t = t.trim().to_string();
    let t = t.chars().collect::<Vec<_>>();

    let mut ret = String::new();

    for i in 0..s.len() - 1 {
        if s[i] == ' ' {
            ret.push(' ');
            continue;
        }

        let val_s = s[i] as i64 - 'a' as i64;
        let val_t = t[i % t.len()] as i64 - 'a' as i64;
        let val = if val_s - val_t > 0 {
            val_s - val_t - 1
        } else {
            val_s - val_t + 25
        };

        ret.push((val as u8 + 'a' as u8) as char);
    }

    writeln!(out, "{ret}").unwrap();
}
