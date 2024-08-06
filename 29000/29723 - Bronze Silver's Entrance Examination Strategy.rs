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

    let (mut n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut subjects = vec![(String::new(), 0); n];
    let mut score = 0;

    for i in 0..n {
        subjects[i] = (scan.token::<String>(), scan.token::<i64>());
    }

    for _ in 0..k {
        let subject_public = scan.token::<String>();

        for i in 0..n {
            if subjects[i].0 == subject_public {
                score += subjects[i].1;
                subjects.remove(i);
                break;
            }
        }

        n -= 1;
    }

    subjects.sort_by(|a, b| a.1.cmp(&b.1));

    let mut min = 0;
    let mut max = 0;

    for i in 0..m - k {
        min += subjects[i].1;
        max += subjects[n - i - 1].1;
    }

    writeln!(out, "{} {}", score + min, score + max).unwrap();
}
