use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let n = s.parse::<usize>().unwrap();
    let mut visited = vec![false; 26];
    let mut ret = String::new();

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        for c in s.chars() {
            if c == ' ' {
                continue;
            }

            let idx = (c as u8 - 'A' as u8) as usize;

            if visited[idx] {
                continue;
            }

            visited[idx] = true;
            ret.push(c);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
