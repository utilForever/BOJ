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

    let n = scan.token::<i64>();
    let mut ret = (0, 0);

    for i in 1..=n {
        let mut cards = [0; 5];

        for j in 0..5 {
            cards[j] = scan.token::<i64>();
        }

        let mut sum = 0;

        for a in 0..5 {
            for b in 0..5 {
                if a == b {
                    continue;
                }

                for c in 0..5 {
                    if a == c || b == c {
                        continue;
                    }

                    sum = sum.max((cards[a] + cards[b] + cards[c]) % 10);
                }
            }
        }

        if sum >= ret.1 {
            ret = (i, sum);
        }
    }

    writeln!(out, "{}", ret.0).unwrap();
}
