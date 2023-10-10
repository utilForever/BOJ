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
    let mut candies_even = Vec::new();
    let mut candies_odd = Vec::new();

    for _ in 0..n {
        let candy = scan.token::<i64>();

        if candy % 2 == 0 {
            candies_even.push(candy);
        } else {
            candies_odd.push(candy);
        }
    }

    candies_odd.sort();

    if candies_even.is_empty() && candies_odd.len() == 1 {
        writeln!(out, "0").unwrap();
    } else if candies_odd.len() % 2 == 1 {
        writeln!(
            out,
            "{}",
            (candies_even.iter().sum::<i64>() + candies_odd.iter().skip(1).sum::<i64>()) / 2
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "{}",
            (candies_even.iter().sum::<i64>() + candies_odd.iter().sum::<i64>()) / 2
        )
        .unwrap();
    }
}
