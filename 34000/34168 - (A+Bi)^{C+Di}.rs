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

const MOD: i64 = 998_244_353;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b, m) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        writeln!(
            out,
            "{}",
            if a == -b {
                if m >= 1 {
                    m % MOD
                } else {
                    0
                }
            } else if b == 0 {
                if a == 1 {
                    (2 * m % MOD + 1) * (2 * m % MOD + 1) % MOD
                } else {
                    (2 * m + 1) % MOD
                }
            } else if a == 0 {
                if b == 1 {
                    ((m / 2) * 2 + 1) % MOD * (2 * m % MOD + 1) % MOD
                } else {
                    ((m / 2) * 2 + 1) % MOD
                }
            } else if a == b {
                ((m / 4) * 2 + 1) % MOD
            } else {
                1
            }
        )
        .unwrap();
    }
}
