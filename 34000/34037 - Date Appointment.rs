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

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut curses = vec![(0, 0); m + 2];

    for i in 1..=m {
        curses[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    curses[m + 1] = (n + 1, 0);

    let mut prefix_sum_penalty = vec![0; m + 2];

    for i in 1..=m {
        prefix_sum_penalty[i] = prefix_sum_penalty[i - 1] + curses[i].1;
    }

    prefix_sum_penalty[m + 1] = prefix_sum_penalty[m];

    let mut dp = vec![i64::MIN / 4; m + 2];
    dp[0] = 0;

    for i in 1..=m + 1 {
        let mut val_max = i64::MIN / 4;

        for j in 0..i {
            let len = curses[i].0 - curses[j].0 - 1;
            let tri = len * (len + 1) / 2;
            let penalty = prefix_sum_penalty[i - 1] - prefix_sum_penalty[j];

            val_max = val_max.max(dp[j] + tri - penalty);
        }

        dp[i] = val_max;
    }

    writeln!(out, "{}", dp[m + 1]).unwrap();
}
