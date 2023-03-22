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

    let mut scores = [0; 50];

    for i in 0..50 {
        scores[i] = scan.token::<i64>();
    }

    let score = scan.token::<i64>();
    let ret = scores.iter().position(|&x| x == score).unwrap_or(0);

    writeln!(
        out,
        "{}",
        match ret + 1 {
            1..=5 => "A+",
            6..=15 => "A0",
            16..=30 => "B+",
            31..=35 => "B0",
            36..=45 => "C+",
            46..=48 => "C0",
            49..=50 => "F",
            _ => unreachable!(),
        }
    )
    .unwrap();
}
