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

    let n = scan.token::<usize>();
    let mut coupon = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut price = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut is_exceed = false;
    let mut ret = vec!['?'; n];

    if coupon.len() < price.len() {
        for _ in 0..price.len() - coupon.len() {
            coupon.insert(0, '0');
        }
    }

    if price.len() < n {
        for _ in 0..n - price.len() {
            price.insert(0, '0');
        }
    }

    for i in 0..n {
        if coupon[i] == '?' {
            ret[i] = '9';

            if ret[i] > price[i] {
                is_exceed = true;
            }
        } else {
            if coupon[i] < price[i] && !is_exceed {
                break;
            }

            ret[i] = coupon[i];

            if coupon[i] > price[i] {
                is_exceed = true;
            }
        }
    }

    if ret.iter().any(|&c| c == '?') {
        writeln!(out, "-1").unwrap();
        return;
    }

    writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
}
