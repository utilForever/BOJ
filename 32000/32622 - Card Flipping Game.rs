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

    let n = scan.token::<usize>();
    let mut cards = vec![0; n];

    for i in 0..n {
        cards[i] = scan.token::<i64>();
    }

    let mut ret = 0;
    let mut len_curr = 1;

    for i in 1..n {
        if cards[i] == cards[i - 1] {
            len_curr += 1;
        } else {
            len_curr = 1;
        }

        ret = ret.max(len_curr);
    }

    ret = ret.max(len_curr);

    let mut left = vec![1; n];
    let mut right = vec![1; n];
    let mut flipped = vec![1; n];

    for i in 1..n {
        left[i] = if cards[i] == cards[i - 1] {
            left[i - 1] + 1
        } else {
            1
        }
    }

    for i in (0..n - 1).rev() {
        right[i] = if cards[i] == cards[i + 1] {
            right[i + 1] + 1
        } else {
            1
        }
    }

    for i in 1..n {
        flipped[i] = if 1 - cards[i] == 1 - cards[i - 1] {
            flipped[i - 1] + 1
        } else {
            1
        }
    }

    for i in 0..n - 1 {
        if cards[i] == 1 - cards[i + 1] {
            let val = flipped[i] + right[i + 1];
            ret = ret.max(val);
        }
    }

    ret = ret.max(flipped[n - 1]);

    writeln!(out, "{ret}").unwrap();
}
