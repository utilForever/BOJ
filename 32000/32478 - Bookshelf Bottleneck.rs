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

    let (n, h_max) = (scan.token::<usize>(), scan.token::<i64>());
    let mut books = vec![[0; 3]; n];
    let mut check = true;

    for i in 0..n {
        let (l, w, h) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if l > h_max && w > h_max && h > h_max {
            check = false;
        }

        books[i] = [l, w, h];
        books[i].sort();
    }

    if check {
        let mut ret = 0;

        for book in books {
            ret += if book[0] <= h_max && book[1] <= h_max {
                book[0]
            } else {
                book[1]
            };
        }

        writeln!(out, "{ret}").unwrap();
    } else {
        writeln!(out, "impossible").unwrap();
    }
}
