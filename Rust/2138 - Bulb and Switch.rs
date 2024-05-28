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
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut ret = i64::MAX;

    let line = scan.token::<String>();

    for (i, c) in line.chars().enumerate() {
        a[i] = c.to_digit(10).unwrap();
    }

    let line = scan.token::<String>();

    for (i, c) in line.chars().enumerate() {
        b[i] = c.to_digit(10).unwrap();
    }

    let mut a_clone = a.clone();
    let mut ret1 = 0;

    // Case 1: When first switch is not pressed
    for i in 1..n {
        if a_clone[i - 1] != b[i - 1] {
            ret1 += 1;

            for j in i - 1..=(i + 1).min(n - 1) {
                a_clone[j] ^= 1;
            }
        }
    }

    if a_clone == b {
        ret = ret.min(ret1);
    }

    // Case 2: When first switch is pressed
    let mut a_clone = a.clone();
    let mut ret2 = 1;
    a_clone[0] ^= 1;
    a_clone[1] ^= 1;

    for i in 1..n {
        if a_clone[i - 1] != b[i - 1] {
            ret2 += 1;

            for j in i - 1..=(i + 1).min(n - 1) {
                a_clone[j] ^= 1;
            }
        }
    }

    if a_clone == b {
        ret = ret.min(ret2);
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}
