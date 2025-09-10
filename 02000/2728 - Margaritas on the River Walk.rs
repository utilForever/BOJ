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

    let n = scan.token::<usize>();

    for _ in 0..n {
        let (v, d) = (scan.token::<usize>(), scan.token::<usize>());
        let mut costs = vec![0; v];

        for i in 0..v {
            costs[i] = scan.token::<usize>();
        }

        let mut cnt = vec![0; d + 1];

        for cost in costs {
            if cost <= d {
                cnt[cost] += 1;
            }
        }

        let mut sum = vec![0; d + 1];
        let mut acc = 0;

        for i in 1..=d {
            acc += i * cnt[i];
            sum[i] = acc;
        }

        let mut dp = vec![0; d + 1];
        dp[0] = 1;

        let mut ret = 0i64;

        for i in (0..=d).rev() {
            let remain = (d - i) as i64;
            let need = remain - (sum[i] as i64);

            if need > 0 {
                let need = need as usize;

                if need <= d {
                    ret += dp[need];
                }
            }

            if i > 0 {
                let times = cnt[i];

                for _ in 0..times {
                    for j in (0..=d - i).rev() {
                        dp[i + j] += dp[j];
                    }
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
