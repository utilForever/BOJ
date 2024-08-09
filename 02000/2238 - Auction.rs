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

    let (u, n) = (scan.token::<usize>(), scan.token::<i64>());
    let mut auctions = vec![Vec::new(); u + 1];

    for _ in 0..n {
        let (s, p) = (scan.token::<String>(), scan.token::<usize>());
        auctions[p].push(s);
    }

    let mut cnt_min = usize::MAX;
    let mut ret = (String::new(), 0);

    for i in 1..=u {
        if auctions[i].is_empty() {
            continue;
        }

        if auctions[i].len() < cnt_min {
            cnt_min = auctions[i].len();
            ret = (auctions[i][0].clone(), i);
        }
    }

    writeln!(out, "{} {}", ret.0, ret.1).unwrap();
}
