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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, r) = (scan.token::<usize>(), scan.token::<i64>());
    let mut grains_of_rice = vec![(0, 0); n];

    for i in 0..n {
        grains_of_rice[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut cnt_max = 0;
    let mut ret = (0, 0);

    for i in -100..=100 {
        for j in -100..=100 {
            let mut cnt = 0;

            for &(x, y) in grains_of_rice.iter() {
                if (x - i as i64).pow(2) + (y - j as i64).pow(2) <= r * r {
                    cnt += 1;
                }
            }

            if cnt > cnt_max {
                cnt_max = cnt;
                ret = (i, j);
            }
        }
    }

    writeln!(out, "{} {}", ret.0, ret.1).unwrap();
}
