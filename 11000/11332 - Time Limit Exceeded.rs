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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let c = scan.token::<i64>();

    for _ in 0..c {
        let (time_complexity, n, t, l) = (
            scan.token::<String>(),
            scan.token::<i128>(),
            scan.token::<i128>(),
            scan.token::<i128>(),
        );
        let time = if time_complexity == "O(N)" {
            n * t
        } else if time_complexity == "O(N^2)" {
            n * n * t
        } else if time_complexity == "O(N^3)" {
            n * n * n * t
        } else if time_complexity == "O(2^N)" {
            if n >= 30 {
                i128::MAX
            } else {
                2i128.pow(n as u32) * t
            }
        } else {
            if n >= 13 {
                i128::MAX
            } else {
                let mut val = 1;

                for i in 1..=n {
                    val *= i;
                }

                val * t
            }
        };

        writeln!(
            out,
            "{}",
            if time <= l * 100_000_000 {
                "May Pass."
            } else {
                "TLE!"
            }
        )
        .unwrap();
    }
}
