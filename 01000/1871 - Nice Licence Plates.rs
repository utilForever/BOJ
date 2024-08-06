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

    for _ in 0..n {
        let num = scan.token::<String>();
        let num = num.chars().collect::<Vec<_>>();
        let first = num.iter().take(3).enumerate().fold(0, |acc, (i, &x)| {
            acc + (x as i64 - 'A' as i64) * 26_i64.pow(2 - i as u32)
        });
        let second = num
            .iter()
            .skip(4)
            .take(4)
            .enumerate()
            .fold(0, |acc, (i, &x)| {
                acc + (x as i64 - '0' as i64) * 10_i64.pow(3 - i as u32)
            });

        writeln!(
            out,
            "{}",
            if (first - second).abs() <= 100 {
                "nice"
            } else {
                "not nice"
            }
        )
        .unwrap();
    }
}
