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

fn lcm(first: i128, second: i128) -> i128 {
    first * second / gcd(first, second)
}

fn gcd(first: i128, second: i128) -> i128 {
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

    let (n, l) = (scan.token::<usize>(), scan.token::<i64>());
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
    }

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    let mut set_zeros = Vec::new();
    let mut set_ones = Vec::new();

    for (&val_a, &val_b) in a.iter().zip(b.iter()) {
        if val_b == 0 {
            set_zeros.push(val_a);
        } else {
            set_ones.push(val_a);
        }
    }

    if set_ones.is_empty() {
        writeln!(out, "{}", if set_zeros.contains(&1) { -1 } else { 1 }).unwrap();
        return;
    }

    let mut val_lcm = set_ones[0] as i128;

    for &val in set_ones.iter().skip(1) {
        val_lcm = lcm(val_lcm, val as i128);

        if val_lcm > l as i128 {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    if val_lcm > l as i128 {
        writeln!(out, "-1").unwrap();
        return;
    }

    for &val in set_zeros.iter() {
        if val_lcm % val as i128 == 0 {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    writeln!(out, "{val_lcm}").unwrap();
}
