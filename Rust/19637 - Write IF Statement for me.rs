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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut titles = Vec::new();
    let mut prev = -1;

    for _ in 0..n {
        let (title, power) = (scan.token::<String>(), scan.token::<i64>());

        if prev == power {
            continue;
        }

        titles.push((title, power));
        prev = power;
    }

    for _ in 0..m {
        let power = scan.token::<i64>();

        match titles.binary_search_by_key(&power, |&(_, p)| p) {
            Ok(i) => writeln!(out, "{}", titles[i].0).unwrap(),
            Err(i) => writeln!(out, "{}", titles[i].0).unwrap(),
        }
    }
}
