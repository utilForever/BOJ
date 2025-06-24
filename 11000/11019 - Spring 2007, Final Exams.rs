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

const GRADES: [f64; 11] = [0.0, 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, h) = (scan.token::<usize>(), scan.token::<usize>());
        let mut times = vec![vec![0; 11]; n];

        for a in 0..n {
            for b in 1..11 {
                times[a][b] = scan.token::<usize>();
            }
        }

        let mut dp = vec![f64::MIN; h + 1];
        dp[0] = 0.0;

        for time in times {
            let mut dp_new = vec![f64::MIN; h + 1];

            for t in 0..=h {
                for j in 0..11 {
                    if t + time[j] > h {
                        continue;
                    }

                    dp_new[t + time[j]] = dp_new[t + time[j]].max(dp[t] + GRADES[j]);
                }
            }

            dp = dp_new;
        }

        writeln!(out, "Data Set {i}:").unwrap();

        let sum_max = dp.into_iter().fold(f64::MIN, |acc, x| acc.max(x));
        let ret = sum_max / n as f64;

        writeln!(out, "{:.2}", (ret * 100.0 + 0.5).floor() / 100.0).unwrap();

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
