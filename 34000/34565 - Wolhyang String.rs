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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let s = scan.token::<String>().chars().collect::<Vec<_>>();

        if n % 3 != 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut dp = vec![vec![0; 2]; n / 3 + 1];
        let mut dp_new = vec![vec![0; 2]; n / 3 + 1];

        dp[0][0] = 1;

        for i in 0..n {
            dp_new.iter_mut().for_each(|x| x.fill(0));

            if s[i] != 'Y' {
                for j in 0..=n / 3 {
                    dp_new[j][0] = (dp_new[j][0] + dp[j][0] + dp[j][1]) % MOD;
                }
            }

            if s[i] != 'H' && i > 0 && i + 1 < n {
                for j in 0..n / 3 {
                    dp_new[j + 1][1] = (dp_new[j + 1][1] + dp[j][0]) % MOD;
                }
            }

            std::mem::swap(&mut dp, &mut dp_new);
        }

        writeln!(out, "{}", dp[n / 3][0]).unwrap();
    }
}
