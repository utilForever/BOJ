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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut stations = vec![String::new(); n];

    for i in 0..n {
        stations[i] = scan.token::<String>();
    }

    let calculate = |val: &Vec<bool>| -> i64 {
        let mut prev = val[0];
        let mut cnt = 1;
        let mut ret = 1;

        for i in 1..val.len() {
            if prev == val[i] {
                cnt = 1;
            } else {
                prev = val[i];
                cnt += 1;
                ret = ret.max(cnt);
            }
        }

        ret
    };

    let mut ret = 0;

    for i in 0..(1 << n) {
        let mut val = Vec::new();

        for j in 0..n {
            val.push((i & (1 << j)) != 0);
        }

        if calculate(&val) == m {
            let mut k = 1;

            for elem in val.iter() {
                k = (k * if *elem { 5 } else { 11 }) % MOD;
            }

            ret = (ret + k) % MOD;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
