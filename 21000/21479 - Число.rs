use io::Write;
use std::{cmp::Ordering, io};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut nums = Vec::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        nums.push(s);
    }

    if nums.iter().all(|s| s == &"0") {
        writeln!(out, "0").unwrap();
        return;
    }

    nums.sort_by(|a, b| {
        let ab = a.to_string() + &b.to_string();
        let ba = b.to_string() + &a.to_string();

        match ab.cmp(&ba) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    });

    let mut ret = String::new();

    for num in nums.iter() {
        ret.push_str(&num);
    }

    writeln!(out, "{ret}").unwrap();
}
