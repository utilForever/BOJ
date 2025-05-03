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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut s = scan.token::<String>();

    s.insert(0, '0');

    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; 27]; n + 1];
    let mut max = vec![vec![0; 26]; 26];

    for i in 1..=n {
        for j in 1..=26 {
            dp[i][j] =
                dp[i - 1][j].max(max[(s[i] as u8 - b'A') as usize][j - 1] - (n as i64 - i as i64));
        }

        if s[i] == 'X' {
            continue;
        }

        for j in 0..26 {
            max[(s[i] as u8 - b'A') as usize][j] =
                max[(s[i] as u8 - b'A') as usize][j].max(dp[i - 1][j] + n as i64 - i as i64 + 1);
        }
    }

    writeln!(out, "{}", n as i64 - dp[n][k.min(26)]).unwrap();
}
