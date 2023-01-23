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
    let mut power = vec![0; n];

    for i in 0..n {
        power[i] = scan.token::<i64>();
    }

    if n == 1 {
        writeln!(out, "{}", power[0]).unwrap();
        return;
    }

    power.sort();

    let mut power_new = vec![0; n];
    let mut idx = 0;
    let mut left = 0;
    let mut right = n - 1;

    while left <= right {
        power_new[idx] = power[right];
        idx += 1;
        right -= 1;

        if left > right {
            break;
        }

        power_new[idx] = power[left];
        idx += 1;
        left += 1;
    }

    let mut ret = power_new[0];

    for i in 1..n {
        ret += (power_new[i] - power_new[i - 1]).max(0);
    }

    writeln!(out, "{ret}").unwrap();
}
