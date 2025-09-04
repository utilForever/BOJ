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

fn check(messages: &Vec<(i64, i64)>, mid: f64) -> bool {
    let mut left = messages[0].0 as f64 - mid;
    let mut right = messages[0].0 as f64 + mid;

    for i in 1..messages.len() {
        let dt = (messages[i].1 - messages[i - 1].1) as f64;

        left -= dt;
        right += dt;

        let pos = messages[i].0 as f64;
        let l = pos - mid;
        let r = pos + mid;

        left = left.max(l);
        right = right.min(r);

        if left > right {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut messages = vec![(0, 0); n];

        for j in 0..n {
            messages[j] = (scan.token::<i64>(), scan.token::<i64>());
        }

        messages.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let min = messages.iter().map(|x| x.0).min().unwrap();
        let max = messages.iter().map(|x| x.0).max().unwrap();

        let mut left = 0.0;
        let mut right = (max - min) as f64 / 2.0;

        for _ in 0..80 {
            let mid = (left + right) / 2.0;

            if check(&messages, mid) {
                right = mid;
            } else {
                left = mid;
            }
        }

        writeln!(out, "Case #{i}: {:.12}", right).unwrap();
    }
}
