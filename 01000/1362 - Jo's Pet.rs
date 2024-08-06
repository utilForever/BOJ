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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut sum = vec![0; 100001];
    sum[0] = 1;

    for i in 2..=100000 {
        let mut num = 2_i64;

        while num * num <= i {
            if i % num == 0 {
                break;
            }

            num += 1;
        }

        if num * num <= i {
            continue;
        }

        for j in i..=100000 {
            sum[j as usize] = (sum[j as usize] + sum[(j - i) as usize]) % MOD;
        }
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", sum[n]).unwrap();
    }
}
