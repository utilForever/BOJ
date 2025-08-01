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

    let mut dp = vec![vec![0u128; 251]; 126];

    for i in 0..=250 {
        dp[0][i] = 1;
    }

    for i in 1..=125 {
        for j in 1..=250 {
            dp[i][j] = dp[i][j - 1] + if i >= j { dp[i - j][j] } else { 0 };
        }
    }

    let mut ret = vec![0; 251];

    for i in 1..=250 {
        let mut val = 0;

        if i % 2 == 0 {
            val += dp[i / 2][i / 2];
        }

        for j in 1..=i {
            if (i - j) % 2 == 0 {
                val += dp[(i - j) / 2][j];
            }
        }

        ret[i] = val;
    }

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        writeln!(out, "{n} {}", ret[n]).unwrap();
    }
}
