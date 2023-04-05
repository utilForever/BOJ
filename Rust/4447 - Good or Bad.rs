use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let mut cnt_g = 0;
        let mut cnt_b = 0;

        for c in s.chars() {
            if c == 'g' || c == 'G' {
                cnt_g += 1;
            } else if c == 'b' || c == 'B' {
                cnt_b += 1;
            }
        }

        writeln!(
            out,
            "{s} is {}",
            if cnt_g > cnt_b {
                "GOOD"
            } else if cnt_g < cnt_b {
                "A BADDY"
            } else {
                "NEUTRAL"
            }
        )
        .unwrap();
    }
}
