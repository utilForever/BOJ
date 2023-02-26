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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<i64>();
    }

    let cnt_limit = ((9 * m) as f64 / 10.0).ceil() as usize;
    let mut cnt = vec![0; 1000001];
    let mut ret = false;

    for i in 0..m {
        cnt[heights[i] as usize] += 1;

        if cnt[heights[i] as usize] >= cnt_limit {
            ret = true;
        }
    }

    if !ret {
        for i in m..n {
            cnt[heights[i - m] as usize] -= 1;
            cnt[heights[i] as usize] += 1;

            if cnt[heights[i] as usize] >= cnt_limit {
                ret = true;
            }
        }
    }

    writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
}
