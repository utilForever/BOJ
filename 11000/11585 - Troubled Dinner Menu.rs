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

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut roulette1 = vec![0; 2 * n - 1];
    let mut roulette2 = vec![0; n];

    for i in 0..n {
        roulette1[i] = scan.token::<char>() as usize;
    }

    for i in 0..n {
        roulette2[i] = scan.token::<char>() as usize;
    }

    for i in 0..n - 1 {
        roulette1[n + i] = roulette1[i];
    }

    let mut cmp = 0;
    let mut fail = vec![0; n];

    for i in 1..n {
        while cmp > 0 && roulette2[cmp] != roulette2[i] {
            cmp = fail[cmp - 1];
        }

        if roulette2[cmp] == roulette2[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    let mut cnt = 0;
    cmp = 0;

    for i in 0..2 * n - 1 {
        while cmp > 0 && roulette1[i] != roulette2[cmp] {
            cmp = fail[cmp - 1];
        }

        if roulette1[i] == roulette2[cmp] {
            if cmp == n - 1 {
                cnt += 1;
                cmp = fail[cmp];
            } else {
                cmp += 1;
            }
        }
    }

    let ret = gcd(cnt, n);
    writeln!(out, "{}/{}", cnt / ret, n / ret).unwrap();
}
