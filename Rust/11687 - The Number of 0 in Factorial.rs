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

    let n = scan.token::<i64>();
    let calculate_cnt_of_five = |n: i64| -> i64 {
        let mut num = 0;
        let mut five = 5;

        while n >= five {
            num += n / five;
            five *= 5;
        }

        num
    };

    let mut left = 5;
    let mut right = 500_000_000;

    while left <= right {
        let mid = (left + right) / 2;
        let cnt_of_five = calculate_cnt_of_five(mid);

        if cnt_of_five >= n {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    if calculate_cnt_of_five(left) == n {
        writeln!(out, "{left}").unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
