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

    let (m, n) = (scan.token::<i64>(), scan.token::<i64>());
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut numbers = Vec::new();

    for i in m..=n {
        let mut word = Vec::new();
        let mut num = i;

        while num > 0 {
            word.push(words[(num % 10) as usize]);
            num /= 10;
        }

        word.reverse();
        numbers.push((word.join(" "), i));
    }

    numbers.sort_by(|a, b| a.0.cmp(&b.0));

    for (idx, (_, num)) in numbers.iter().enumerate() {
        write!(out, "{num} ").unwrap();

        if (idx + 1) % 10 == 0 {
            writeln!(out).unwrap();
        }
    }
}
