use io::Write;
use std::{collections::BTreeSet, io, str};

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

    let n = scan.token::<usize>();
    let mut balls = vec![0; n];

    for i in 0..n {
        balls[i] = scan.token::<i64>();
    }

    if n == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut ret = BTreeSet::new();

    for i in 0..n {
        if i < n.saturating_sub(1) && balls[i] == balls[i + 1] {
            ret.insert(balls[i]);
        }

        if i < n.saturating_sub(2) && balls[i] == balls[i + 2] {
            ret.insert(balls[i]);
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();
}
