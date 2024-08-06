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

fn count_divisor(num: i64, divisor: i64) -> i64 {
    let mut cnt = 0;
    let mut div = divisor;

    while div <= num {
        cnt += num / div;
        div *= divisor;
    }

    cnt
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    
    let (cnt_n_2, cnt_n_5) = (count_divisor(n, 2), count_divisor(n, 5));
    let (cnt_m_2, cnt_m_5) = (count_divisor(m, 2), count_divisor(m, 5));
    let (cnt_n_minus_m_2, cnt_n_minus_m_5) = (count_divisor(n - m, 2), count_divisor(n - m, 5));

    let cnt_2 = (cnt_n_2 - cnt_m_2 - cnt_n_minus_m_2).max(0);
    let cnt_5 = (cnt_n_5 - cnt_m_5 - cnt_n_minus_m_5).max(0);

    writeln!(out, "{}", cnt_2.min(cnt_5)).unwrap();
}
