use io::Write;
use std::io;

const N: usize = 149;
const Q: i64 = 13;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "{N}").unwrap();

    for i in 0..N {
        let x = (i / Q as usize) as i64;
        let y = (i % Q as usize) as i64;

        for j in 0..N {
            let a = (j / Q as usize) as i64;
            let b = (j % Q as usize) as i64;
            let v = (a * x + b - y).rem_euclid(Q);

            write!(out, "{}", if v == 0 { 'O' } else { '.' }).unwrap();
        }

        writeln!(out).unwrap();
    }
}
