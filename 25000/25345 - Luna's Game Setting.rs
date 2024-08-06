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

fn calculate_combination(n: usize, k: usize) -> i64 {
    let mut factor = vec![1; n + 1];

    for i in 2..=n {
        factor[i] = (factor[i - 1] * i as i64) % 1_000_000_007;
    }

    let divide = factor[k] * factor[n - k] % 1_000_000_007;
    let mut rev = 1;
    let mut multiplier = divide;
    let mut count = 1_000_000_005;

    while count > 0 {
        if count % 2 == 1 {
            rev = (rev * multiplier) % 1_000_000_007;
            count -= 1;
        } else {
            multiplier = (multiplier * multiplier) % 1_000_000_007;
            count /= 2;
        }
    }

    factor[n] * rev % 1_000_000_007
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    for _ in 0..n {
        _ = scan.token::<i64>();
    }

    let mut power = vec![1; k];

    for i in 1..k {
        power[i] = power[i - 1] * 2 % 1_000_000_007;
    }

    writeln!(
        out,
        "{}",
        calculate_combination(n, k) * power[k - 1] % 1_000_000_007
    )
    .unwrap();
}
