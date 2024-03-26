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
        let n = scan.token::<usize>();
        let mut students = vec![(0, 0); 1005];

        for i in 1..=n {
            students[i] = (scan.token::<i64>(), (scan.token::<f64>() * 100.0 + 0.5) as i64);
        }

        let mut ret = 0;

        for f in 1..=1004 {
            let mut dp = vec![0; 1005];

            for i in 1..=n {
                for j in (students[i].1 as usize..=f).rev() {
                    dp[j] = dp[j].max(
                        dp[j - students[i].1 as usize]
                            + students[i].0 * (10000 - students[i].1 * (f as i64 - students[i].1)),
                    );
                }
            }

            ret = ret.max(dp[f]);
        }

        write!(out, "{}", ret / 10000).unwrap();

        match ret % 10000 {
            val @ 0..=9 => {
                writeln!(out, ".000{val}00000").unwrap();
            }
            val @ 10..=99 => {
                writeln!(out, ".00{val}00000").unwrap();
            }
            val @ 100..=999 => {
                writeln!(out, ".0{val}00000").unwrap();
            }
            val @ 1000..=9999 => {
                writeln!(out, ".{val}00000").unwrap();
            }
            _ => unreachable!(),
        }
    }
}
