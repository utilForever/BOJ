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

// Reference: https://casterian.net/algo/ext-euclidean.html
fn process_extended_euclidean(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (gcd, s, t) = process_extended_euclidean(b, a % b);
    (gcd, t, s - (a / b) * t)
}

fn floor_div(a: i128, b: i128) -> i128 {
    if a < 0 {
        if a % b == 0 {
            return a / b;
        } else {
            return a / b - 1;
        }
    }

    a / b
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (a, b, s) = (
        scan.token::<i128>(),
        scan.token::<i128>(),
        scan.token::<i128>(),
    );

    if a == 0 && b == 0 {
        writeln!(out, "{}", if s == 0 { "YES" } else { "NO" }).unwrap();
        return;
    } else if a == 0 {
        writeln!(out, "{}", if s % b == 0 { "YES" } else { "NO" }).unwrap();
        return;
    } else if b == 0 {
        writeln!(out, "{}", if s % a == 0 { "YES" } else { "NO" }).unwrap();
        return;
    }

    if a == s || b == s {
        writeln!(out, "YES").unwrap();
        return;
    }

    let (gcd, c, d) = process_extended_euclidean(a, b);

    if s % gcd != 0 {
        writeln!(out, "NO").unwrap();
        return;
    }

    let c = c * (s / gcd);
    let d = d * (s / gcd);

    for i in floor_div(-gcd * c, b) + 1..floor_div(gcd * d, a) + 1 {
        if process_extended_euclidean(c + (floor_div(i * b, gcd)), d - (floor_div(i * a, gcd))).0
            == 1
        {
            writeln!(out, "YES").unwrap();
            return;
        }
    }

    writeln!(out, "NO").unwrap();
}
