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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut iqs_computer_science = vec![0; n];
        let mut iqs_economics = vec![0; m];

        for i in 0..n {
            iqs_computer_science[i] = scan.token::<i64>();
        }

        for i in 0..m {
            iqs_economics[i] = scan.token::<i64>();
        }

        let sum_computer_science = iqs_computer_science.iter().sum::<i64>();
        let sum_economics = iqs_economics.iter().sum::<i64>();
        let avg_computer_science = sum_computer_science as f64 / n as f64;
        let avg_economics = sum_economics as f64 / m as f64;
        let mut ret = 0;

        for i in 0..n {
            let avg_computer_science_new =
                (sum_computer_science - iqs_computer_science[i]) as f64 / (n - 1) as f64;
            let avg_economics_new =
                (sum_economics + iqs_computer_science[i]) as f64 / (m + 1) as f64;

            if avg_computer_science_new > avg_computer_science && avg_economics_new > avg_economics
            {
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
