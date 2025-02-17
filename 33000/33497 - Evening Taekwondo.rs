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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut classes = vec![(0, 0); m];

    for i in 0..m {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        if a + b > n as i64 {
            writeln!(out, "NO").unwrap();
            return;
        }

        classes[i] = (a, b);
    }

    let sum = classes.iter().map(|(a, b)| a + b).sum::<i64>();

    if sum > (n * m - n) as i64 {
        writeln!(out, "NO").unwrap();
        return;
    }

    writeln!(out, "YES").unwrap();

    for i in 0..n {
        let mut use_card = false;

        for j in 0..m {
            if classes[j].0 + classes[j].1 < n as i64 - i as i64 && !use_card {
                write!(out, "X").unwrap();
                use_card = true;
            } else if classes[j].0 > 0 {
                write!(out, "+").unwrap();
                classes[j].0 -= 1;
            } else {
                write!(out, "-").unwrap();
                classes[j].1 -= 1;
            }
        }

        writeln!(out).unwrap();
    }
}
