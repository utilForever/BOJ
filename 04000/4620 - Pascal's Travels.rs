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

    loop {
        let n = scan.token::<i64>();

        if n == -1 {
            break;
        }

        let n = n as usize;
        let mut board = vec![vec![0; n]; n];
        let mut num_path = vec![vec![0_i64; n]; n];

        for i in 0..n {
            let s = scan.token::<String>();

            for (idx, c) in s.chars().enumerate() {
                board[i][idx] = c as i64 - '0' as i64;
            }
        }

        num_path[0][0] = 1;

        for i in 0..n {
            for j in 0..n {
                let num = board[i][j];

                if num == 0 {
                    continue;
                }

                if (num + i as i64) < n as i64 {
                    num_path[num as usize + i][j] += num_path[i][j];
                }

                if (num + j as i64) < n as i64 {
                    num_path[i][num as usize + j] += num_path[i][j];
                }
            }
        }

        writeln!(out, "{}", num_path[n - 1][n - 1]).unwrap();
    }
}
