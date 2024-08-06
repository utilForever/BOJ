use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "n e").unwrap();
    writeln!(out, "- -----------").unwrap();

    let mut ret = 0.0;

    for i in 0..=9 {
        ret += 1.0 / (1..=i).product::<i64>() as f64;

        if i <= 1 {
            writeln!(out, "{i} {:.0}", ret).unwrap();
        } else if i == 2 {
            writeln!(out, "{i} {:.1}", ret).unwrap();
        } else {
            writeln!(out, "{i} {:.9}", ret).unwrap();
        }
    }
}
