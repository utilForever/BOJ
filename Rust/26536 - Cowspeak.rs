use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim().to_string();
        let words = s.split_whitespace().collect::<Vec<_>>();
        let mut ret = String::new();

        for word in words {
            let mut cnt_m = 0;
            let mut cnt_o = 0;

            for c in word.chars() {
                if c == 'M' {
                    cnt_m += 1;
                } else if c == 'O' {
                    cnt_o += 1;
                }
            }

            ret.push((cnt_m * 16 + cnt_o) as u8 as char);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
