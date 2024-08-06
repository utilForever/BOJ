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

fn calculate_nums(sum: &mut i128, mut n: i128, cnt: i128) {
    while n > 0 {
        *sum += cnt * (n % 10);
        n /= 10;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (mut a, mut b) = (scan.token::<i128>(), scan.token::<i128>());
        let mut multiplier = 1;
        let mut sum = 0;

        if a == 0 {
            if b == 0 {
                writeln!(out, "{}", 0).unwrap();
                continue;
            } else {
                a = 1;
            }
        }

        while a <= b {
            while a % 10 != 0 && a <= b {
                calculate_nums(&mut sum, a, multiplier);
                a += 1;
            }

            if a > b {
                break;
            }

            while b % 10 != 9 && a <= b {
                calculate_nums(&mut sum, b, multiplier);
                b -= 1;
            }

            let cnt = (b / 10) - (a / 10) + 1;

            for i in 0..10 {
                sum += cnt * multiplier * i;
            }

            a /= 10;
            b /= 10;
            multiplier *= 10;
        }

        writeln!(out, "{}", sum).unwrap();
    }
}
