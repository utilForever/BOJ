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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let n = scan.token::<i64>();

    for i in 1..=n {
        let s = scan.token::<i64>();
        let mut reds = Vec::new();
        let mut blues = Vec::new();

        for _ in 0..s {
            let segment = scan.token::<String>().chars().collect::<Vec<_>>();

            if *segment.last().unwrap() == 'R' {
                let val = segment[..segment.len() - 1].iter().collect::<String>();
                reds.push(val.parse::<i64>().unwrap());
            } else {
                let val = segment[..segment.len() - 1].iter().collect::<String>();
                blues.push(val.parse::<i64>().unwrap());
            }
        }

        if reds.is_empty() || blues.is_empty() {
            writeln!(out, "Case #{i}: 0").unwrap();
            continue;
        }

        reds.sort_unstable_by(|a, b| b.cmp(a));
        blues.sort_unstable_by(|a, b| b.cmp(a));

        let mut ret = 0;

        for (&r, &b) in reds.iter().zip(blues.iter()) {
            if r == 0 || b == 0 {
                ret += r.max(b) - 1;
                break;
            }

            ret += r + b - 2;
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
