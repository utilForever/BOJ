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

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut shops = vec![0; 50];

        if n % 2 == 0 {
            for i in 0..50 {
                shops[i] = (i + 1) * 2;
            }
        } else {
            for i in 0..50 {
                shops[i] = i * 2 + 1;
            }
        }

        let mut left = 0;
        let mut right = 49;
        let mut ret = 1;

        while left <= right {
            let mid = (left + right) / 2;

            if shops[mid] < n {
                left = mid + 1;
            } else if shops[mid] > n {
                right = mid - 1;
            } else {
                break;
            }

            ret += 1;
        }

        if n % 2 == 1 {
            ret += 1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
