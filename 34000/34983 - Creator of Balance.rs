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

    let (n, ri, rg) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut dp = vec![vec![i64::MAX / 4; rg + 1]; ri + 1];

    dp[0][0] = 0;

    for _ in 0..n {
        let (i, g) = (scan.token::<usize>(), scan.token::<usize>());

        for a in (0..=ri).rev() {
            for b in (0..=rg).rev() {
                if dp[a][b] == i64::MAX / 4 {
                    continue;
                }

                let a_next = (a + i).min(ri);
                let b_next = (b + g).min(rg);

                dp[a_next][b_next] = dp[a_next][b_next].min(dp[a][b] + 1);
            }
        }
    }

    writeln!(
        out,
        "{}",
        if dp[ri][rg] == i64::MAX / 4 {
            -1
        } else {
            dp[ri][rg]
        }
    )
    .unwrap();
}
