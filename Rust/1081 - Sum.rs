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

fn calculate_nums(sum: &mut i64, mut n: i64, cnt: i64) {
    while n > 0 {
        *sum += cnt * (n % 10);
        n /= 10;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut start, mut end) = (scan.token::<i64>(), scan.token::<i64>());
    let mut multiplier = 1;
    let mut sum = 0;

    if start == 0 {
        if end == 0 {
            writeln!(out, "{}", 0).unwrap();
            return;
        } else {
            start = 1;
        }
    }

    while start <= end {
        while start % 10 != 0 && start <= end {
            calculate_nums(&mut sum, start, multiplier);
            start += 1;
        }

        if start > end {
            break;
        }

        while end % 10 != 9 && start <= end {
            calculate_nums(&mut sum, end, multiplier);
            end -= 1;
        }

        let cnt = (end / 10) - (start / 10) + 1;

        for i in 0..10 {
            sum += cnt * multiplier * i;
        }

        start /= 10;
        end /= 10;
        multiplier *= 10;
    }

    writeln!(out, "{}", sum).unwrap();
}
