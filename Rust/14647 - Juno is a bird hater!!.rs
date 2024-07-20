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
    let mut bingo = vec![vec![0; m]; n];
    let mut total = 0;

    for i in 0..n {
        for j in 0..m {
            bingo[i][j] = scan.token::<i32>();

            let mut num = bingo[i][j];
            let mut cnt = 0;

            while num > 0 {
                if num % 10 == 9 {
                    cnt += 1;
                }

                num /= 10;
            }

            total += cnt;
        }
    }

    let mut cnt_max = 0;

    for i in 0..n {
        let mut cnt = 0;

        for j in 0..m {
            let mut num = bingo[i][j];

            while num > 0 {
                if num % 10 == 9 {
                    cnt += 1;
                }

                num /= 10;
            }
        }

        cnt_max = cnt_max.max(cnt);
    }

    for j in 0..m {
        let mut cnt = 0;

        for i in 0..n {
            let mut num = bingo[i][j];

            while num > 0 {
                if num % 10 == 9 {
                    cnt += 1;
                }

                num /= 10;
            }
        }

        cnt_max = cnt_max.max(cnt);
    }

    writeln!(out, "{}", total - cnt_max).unwrap();
}
