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

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut tracking = vec![0; m + 1];
    let mut ret = 0;

    for i in 1..=m {
        let command = scan.token::<i64>();

        if command == 1 {
            let x = scan.token::<i64>();
            ret = (ret + x) % n;
        } else if command == 2 {
            let y = scan.token::<i64>();
            ret -= y;

            if ret < 0 {
                let q = -ret / n;
                ret = n * (q + 1) + ret;
            }

            ret %= n;
        } else {
            let z = scan.token::<usize>();
            ret = tracking[z];
        }

        tracking[i] = ret;
    }

    writeln!(out, "{}", ret + 1).unwrap();
}
