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

fn check(lines: &Vec<usize>, lan_len: usize, minimum_lan_cnt: usize) -> bool {
    let mut cnt = 0;

    for i in 0..lines.len() {
        cnt += lines[i] / lan_len;
    }

    cnt >= minimum_lan_cnt
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (k, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut lines = vec![0; k];

    for i in 0..k {
        lines[i] = scan.token::<usize>();
    }

    let mut l = 1;
    let mut r = *lines.iter().max().unwrap();
    let mut ans = 0;

    while l <= r {
        let mid = (l + r) / 2;

        if check(&lines, mid, n) {
            if ans < mid {
                ans = mid;
            }

            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
