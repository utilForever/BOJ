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

fn pow(x: i64, mut y: i64, m: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % m;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % m;
        }

        piv = piv * piv % m;
        y >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );

    let mut is_prime = vec![false; n + 1];
    is_prime[1] = true;

    let mut nums = Vec::new();
    let mut cnts = HashMap::new();

    for i in 2..=n {
        if is_prime[i] {
            continue;
        }

        nums.push(i);

        let mut j = i * 2;

        while j <= n {
            is_prime[j] = true;
            j += i;
        }
    }

    for &num in nums.iter() {
        let mut temp = num;

        while temp <= n {
            let cnt = n / temp - k / temp - (n - k) / temp;

            *cnts.entry(num).or_insert(0) += cnt as i64;
            temp *= num;
        }
    }

    let mut ret = 1;

    for (&key, &val) in cnts.iter() {
        ret = (ret * pow(key as i64, val, m)) % m;
    }

    writeln!(out, "{ret}").unwrap();
}
