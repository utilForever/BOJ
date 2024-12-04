use io::Write;
use std::{collections::HashMap, io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let s = scan.token::<String>();
    let mut cnt_alphabets = HashMap::new();

    for c in s.chars() {
        *cnt_alphabets.entry(c).or_insert(0) += 1;
    }

    // C(n, 2) = n * (n - 1) / 2
    let cnt_total = n * (n - 1) / 2 % MOD;
    let mut cnt_sum = 0;

    for &count in cnt_alphabets.values() {
        let cnt_local = pow(2, count);
        let cnt_less_than_2 = (1 + count + count * (count - 1) / 2) % MOD;
        let cnt_greater_than_3 = (cnt_local + MOD - cnt_less_than_2) % MOD;
        
        cnt_sum = (cnt_sum + cnt_greater_than_3) % MOD;
    }

    let ret = (cnt_total + cnt_sum) % MOD;

    writeln!(out, "{ret}").unwrap();
}
