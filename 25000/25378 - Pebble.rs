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
    let mut stones = vec![0; n + 1];

    for i in 1..=n {
        stones[i] = scan.token::<i64>();
    }

    let mut can_only_one = vec![vec![false; n + 1]; n + 1];

    for i in 1..=n {
        let mut remain = stones[i];
        let mut check = true;

        for j in i + 1..=n {
            if check && remain == stones[j] {
                can_only_one[i][j] = true;
            }

            if remain > stones[j] {
                check = false;
            }

            remain = if remain > stones[j] {
                remain - stones[j]
            } else {
                stones[j] - remain
            };
        }
    }

    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        dp[i] = dp[i - 1] + 1;

        for j in 1..i {
            if can_only_one[j][i] {
                let candidate = dp[j - 1] + (i - j) as i64;
                dp[i] = dp[i].min(candidate);
            }
        }
    }

    writeln!(out, "{}", dp[n]).unwrap();
}
