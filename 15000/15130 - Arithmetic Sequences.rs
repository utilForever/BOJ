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

    let mut nums = [0; 10];
    let mut a = (0, 0);
    let mut b = (0, 0);

    for i in 0..10 {
        nums[i] = scan.token::<i64>();

        if nums[i] != 0 {
            if a.0 == 0 {
                a = (nums[i], i as i64);
            } else {
                b = (nums[i], i as i64);
            }
        }
    }

    if (b.0 - a.0) % (b.1 - a.1) != 0 {
        writeln!(out, "-1").unwrap();
    } else {
        let d = (b.0 - a.0) / (b.1 - a.1);
        let start = a.0 - a.1 * d;

        for idx in 0..10 {
            write!(out, "{} ", start + idx * d).unwrap();
        }

        writeln!(out).unwrap();
    }
}
