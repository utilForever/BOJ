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
    let mut properties = vec![0; n];

    for i in 0..n {
        properties[i] = scan.token::<i64>();
    }

    properties.sort();

    let mut sum = i64::MAX;
    let mut ans = (0, 0);
    let mut left = 0;
    let mut right = n - 1;

    while left < right {
        let ret = properties[left] + properties[right];

        if ret.abs() < sum {
            sum = ret.abs();
            ans = (properties[left], properties[right]);
        }

        if ret < 0 {
            left += 1;
        } else if ret > 0 {
            right -= 1;
        } else {
            break;
        }
    }

    writeln!(out, "{}", ans.0 + ans.1).unwrap();
}
