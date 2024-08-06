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
        let s = s.split_whitespace().collect::<Vec<_>>();
        let mut ret = s[0].parse::<f64>().unwrap();

        for i in 1..s.len() {
            match s[i] {
                "@" => ret *= 3.0,
                "%" => ret += 5.0,
                "#" => ret -= 7.0,
                _ => unreachable!(),
            }
        }

        writeln!(out, "{:.2}", ret).unwrap();
    }
}
