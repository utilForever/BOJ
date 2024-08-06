use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<i64>().unwrap();

    for i in 1..=t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let n = s.trim().parse::<usize>().unwrap();
        let mut names = vec![String::new(); n];

        for j in 0..n {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            names[j] = s.trim().to_string();
        }

        let mut ret = 0;
        let mut idx = 0;

        for j in 1..n {
            if names[j] < names[idx] {
                ret += 1;
            } else {
                idx = j;
            }
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
