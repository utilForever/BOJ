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

    let s = scan.token::<String>();
    let mut k_cnt = 0;
    let mut k_inv = 0;
    let mut c_cnt = 0;

    let mut p_cnt = 0;
    let mut p_total = 0;
    let mut p_sum_before_c = 0;

    for (idx, c) in s.chars().enumerate() {
        match c {
            'K' => {
                k_inv += idx as i64 - k_cnt;
                k_cnt += 1;
            }
            'P' => {
                p_cnt += 1;
                p_total += 1;
            }
            'C' => {
                p_sum_before_c += p_cnt;
                c_cnt += 1;
            }
            _ => unreachable!(),
        }
    }

    let mut prefix_sum = 0;
    let mut p_cnt = 0;
    let mut c_idx = 0;

    let constant = k_inv + c_cnt * p_total - p_sum_before_c;
    let mut ret = i64::MAX;

    for c in s.chars() {
        match c {
            'P' => {
                p_cnt += 1;
            }
            'C' => {
                c_idx += 1;
                prefix_sum += p_cnt;

                if c_idx >= 1 && c_idx <= c_cnt - 1 {
                    let cost = constant + 2 * prefix_sum - c_idx * p_total;
                    ret = ret.min(cost);
                }
            }
            _ => {}
        }
    }

    writeln!(out, "{ret}").unwrap();
}
