use io::Write;
use std::{collections::BTreeSet, io, str};

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

    let n = scan.token::<usize>();
    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        let mut set = BTreeSet::new();

        for j in 0..i {
            let left = (j as i64 - 2).max(0);
            let right = (i as i64 - j as i64 - 3).max(0);
            let grundy = dp[left as usize] ^ dp[right as usize];

            set.insert(grundy);
        }

        while set.contains(&dp[i]) {
            dp[i] += 1;
        }
    }

    writeln!(out, "{}", if dp[n] > 0 { 1 } else { 2 }).unwrap();
}
