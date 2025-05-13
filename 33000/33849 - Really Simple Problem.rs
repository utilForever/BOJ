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

fn gcd(first: i64, second: i64) -> i64 {
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
        std::mem::swap(&mut min, &mut max);
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
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        x[i] = scan.token::<i64>();
    }

    for i in 0..n {
        y[i] = scan.token::<i64>();
    }

    let mut best_x = x[0];
    let mut best_y = y[0];

    for i in 1..n {
        if y[i] * best_x > best_y * x[i] {
            best_x = x[i];
            best_y = y[i];
        }
    }

    let mut ret = 0;
    let mut cnt = 0;

    for i in 0..n {
        if y[i] * best_x == best_y * x[i] {
            cnt += 1;
        } else {
            ret = ret.max(cnt);
            cnt = 0;
        }
    }

    ret = ret.max(cnt);

    let g = gcd(best_x, best_y);

    writeln!(out, "{} {}", best_y / g, best_x / g).unwrap();
    writeln!(out, "{ret}").unwrap();
}
