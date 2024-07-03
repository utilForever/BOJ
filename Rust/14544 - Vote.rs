use io::Write;
use std::{collections::HashMap, io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let p = scan.token::<i64>();

    for i in 1..=p {
        let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
        let mut candidates = HashMap::new();

        for _ in 0..n {
            candidates.insert(scan.token::<String>(), 0);
        }

        for _ in 0..m {
            let (name, space, _) = (
                scan.token::<String>(),
                scan.token::<i64>(),
                scan.token::<String>(),
            );
            candidates.entry(name).and_modify(|e| *e += space);
        }

        let mut candidates = candidates.iter().collect::<Vec<_>>();
        candidates.sort_by(|a, b| b.1.cmp(a.1));

        write!(out, "VOTE {i}: ").unwrap();

        if candidates.len() == 1 || candidates[0].1 != candidates[1].1 {
            writeln!(out, "THE WINNER IS {} {}", candidates[0].0, candidates[0].1).unwrap();
        } else {
            writeln!(out, "THERE IS A DILEMMA").unwrap();
        }
    }
}
