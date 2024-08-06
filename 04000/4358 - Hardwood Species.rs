use io::Write;
use std::{collections::BTreeMap, io};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut map = BTreeMap::new();
    let mut cnt = 0;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        map.entry(s).and_modify(|e| *e += 1).or_insert(1);
        cnt += 1;
    }

    for (k, v) in map.iter() {
        writeln!(out, "{k} {:.4}", *v as f64 / cnt as f64 * 100.0).unwrap();
    }
}
