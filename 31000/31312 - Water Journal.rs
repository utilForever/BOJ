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

    let (n, a, b) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut is_min_exist = false;
    let mut is_max_exist = false;

    for _ in 0..n - 1 {
        let w = scan.token::<i64>();

        if w == a {
            is_min_exist = true;
        } else if w == b {
            is_max_exist = true;
        }
    }

    if is_min_exist && is_max_exist {
        for i in 1..=n {
            writeln!(out, "{i}").unwrap();
        }
    } else if is_min_exist {
        writeln!(out, "{b}").unwrap();
    } else if is_max_exist {
        writeln!(out, "{a}").unwrap();
    } else {
        writeln!(out, "-1").ok();
    }
}
