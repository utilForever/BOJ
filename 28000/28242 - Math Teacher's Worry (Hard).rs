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
    let mut a = 1;

    while a * a <= n {
        if n % a != 0 {
            a += 1;
            continue;
        }

        let c = n / a;
        let mut b = 1;

        while b * b <= n + 2 {
            if (n + 2) % b != 0 {
                b += 1;
                continue;
            }

            let d = (n + 2) / b;

            if a * d + b * c == n + 1 {
                writeln!(out, "{a} {b} {c} {d}").unwrap();
                return;
            }

            if a * d - b * c == n + 1 {
                writeln!(out, "{a} {} {c} {d}", -b).unwrap();
                return;
            }

            if -a * d + b * c == n + 1 {
                writeln!(out, "{a} {b} {c} {}", -d).unwrap();
                return;
            }

            if -a * d - b * c == n + 1 {
                writeln!(out, "{a} {} {c} {}", -b, -d).unwrap();
                return;
            }

            b += 1;
        }

        a += 1;
    }

    writeln!(out, "-1").unwrap();
}
