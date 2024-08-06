use io::Write;
use std::{cmp, io, str};

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
    let mut a = vec![0; n + 2];

    for i in 0..n {
        a[i] = scan.token::<usize>();
    }

    let mut min_amount = 0;

    for i in 0..n {
        if a[i + 1] > a[i + 2] {
            let count = cmp::min(a[i], a[i + 1] - a[i + 2]);
            a[i] -= count;
            a[i + 1] -= count;
            min_amount += 5 * count;

            let count = cmp::min(a[i], cmp::min(a[i + 1], a[i + 2]));
            a[i] -= count;
            a[i + 1] -= count;
            a[i + 2] -= count;
            min_amount += 7 * count;
        } else {
            let count = cmp::min(a[i], cmp::min(a[i + 1], a[i + 2]));
            a[i] -= count;
            a[i + 1] -= count;
            a[i + 2] -= count;
            min_amount += 7 * count;

            let count = cmp::min(a[i], a[i + 1]);
            a[i] -= count;
            a[i + 1] -= count;
            min_amount += 5 * count;
        }

        let count = a[i];
        a[i] = 0;
        min_amount += 3 * count;
    }

    writeln!(out, "{min_amount}").unwrap();
}
