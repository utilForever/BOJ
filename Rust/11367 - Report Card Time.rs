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

fn calculate_grade(score: i32) -> &'static str {
    match score {
        0..=59 => "F",
        60..=66 => "D",
        67..=69 => "D+",
        70..=76 => "C",
        77..=79 => "C+",
        80..=86 => "B",
        87..=89 => "B+",
        90..=96 => "A",
        97..=100 => "A+",
        _ => panic!("Invalid score"),
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    for _ in 0..n {
        let (a, b) = (scan.token::<String>(), scan.token::<i32>());
        writeln!(out, "{} {}", a, calculate_grade(b)).unwrap();
    }
}
