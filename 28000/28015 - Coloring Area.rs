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
    let mut picture = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            picture[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = 0;

    for i in 0..n {
        let mut cnt_1 = 0;
        let mut cnt_2 = 0;
        let mut val = 0;

        for j in 0..m {
            if val != picture[i][j] {
                if picture[i][j] == 1 {
                    cnt_1 += 1;
                } else if picture[i][j] == 2 {
                    cnt_2 += 1;
                } else {
                    ret += cnt_1.min(cnt_2) + 1;
                    cnt_1 = 0;
                    cnt_2 = 0;
                }

                val = picture[i][j];
            }
        }

        if cnt_1 != 0 || cnt_2 != 0 {
            ret += cnt_1.min(cnt_2) + 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
