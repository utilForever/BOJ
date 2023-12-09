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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut sticks = vec![0; n];

    for i in 0..n {
        sticks[i] = scan.token::<usize>();
    }

    let q = scan.token::<usize>();
    let mut lengths = vec![0; q];

    for i in 0..q {
        lengths[i] = scan.token::<usize>();
    }

    let length_max = *lengths.iter().chain(sticks.iter()).max().unwrap();
    let mut dp = vec![0; length_max + 1];

    for val in sticks {
        dp[val] += 1;
    }

    for i in 1..=length_max {
        let mut j = i * 2;

        while j <= length_max {
            dp[j] += dp[i];
            j += i;
        }
    }

    for length in lengths {
        write!(out, "{} ", dp[length]).unwrap();
    }

    writeln!(out).unwrap();
}
