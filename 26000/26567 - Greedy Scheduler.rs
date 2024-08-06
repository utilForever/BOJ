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

    let m = scan.token::<i64>();

    for _ in 0..m {
        let (n, c) = (scan.token::<usize>(), scan.token::<usize>());
        let mut cashers = vec![0; n];

        for _ in 0..c {
            let t = scan.token::<i64>();
            let min = cashers.iter().min().unwrap();
            let pos = cashers.iter().position(|&x| x == *min).unwrap();

            cashers[pos] += t;

            write!(out, "{} ", pos + 1).unwrap();
        }

        writeln!(out).unwrap();
    }
}
