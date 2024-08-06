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
    let mut lengths = vec![0; 3];

    for i in 1..=n {
        (lengths[0], lengths[1], lengths[2]) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        lengths.sort();

        writeln!(out, "Scenario #{}:", i).unwrap();
        writeln!(
            out,
            "{}",
            if lengths[0] * lengths[0] + lengths[1] * lengths[1] == lengths[2] * lengths[2] {
                "yes"
            } else {
                "no"
            }
        )
        .unwrap();

        if i != n {
            writeln!(out).unwrap();
        }
    }
}
