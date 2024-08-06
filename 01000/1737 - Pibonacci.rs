use io::Write;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    io, str,
};

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

#[derive(Debug, Clone, Copy)]
struct Float {
    a: f64,
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}
impl Eq for Float {}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::mem::transmute::<f64, u64>(self.a).hash(state);
        }
    }
}

fn pibonacci(num: Float, dp: &mut HashMap<Float, i128>) -> i128 {
    if num.a >= 0.0 && num.a <= std::f64::consts::PI {
        return 1;
    }

    if dp.contains_key(&num) {
        return dp[&num];
    }

    let val = pibonacci(Float { a: num.a - 1.0 }, dp)
        + pibonacci(
            Float {
                a: num.a - std::f64::consts::PI,
            },
            dp,
        );
    dp.insert(num, val);

    dp[&num] % 1_000_000_000_000_000_000
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut dp = HashMap::new();

    writeln!(out, "{}", pibonacci(Float { a: n as f64 }, &mut dp)).unwrap();
}
