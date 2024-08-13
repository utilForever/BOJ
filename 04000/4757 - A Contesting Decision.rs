use io::Write;
use std::{io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut teams = vec![(String::new(), 0, 0); n];

    for i in 0..n {
        let name = scan.token::<String>();
        let mut solved = 0;
        let mut penalty = 0;

        for _ in 0..4 {
            let (submission, time) = (scan.token::<i64>(), scan.token::<i64>());

            if time == 0 {
                continue;
            }

            solved += 1;
            penalty += time + (submission - 1) * 20;
        }

        teams[i] = (name, solved, penalty);
    }

    teams.sort_by(|a, b| {
        if a.1 != b.1 {
            return b.1.cmp(&a.1);
        }

        if a.2 != b.2 {
            return a.2.cmp(&b.2);
        }

        a.0.cmp(&b.0)
    });

    writeln!(out, "{} {} {}", teams[0].0, teams[0].1, teams[0].2).unwrap();
}
