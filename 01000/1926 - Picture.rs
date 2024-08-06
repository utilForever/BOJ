use io::Write;
use std::{collections::VecDeque, io, str};

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

    let mut queue = VecDeque::new();
    let mut checked = vec![vec![false; m]; n];
    let mut ret_cnt = 0;
    let mut ret_area = 0;

    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    for i in 0..n {
        for j in 0..m {
            let mut cnt = 0;

            if picture[i][j] == 1 && !checked[i][j] {
                checked[i][j] = true;
                queue.push_back((i, j));
                ret_cnt += 1;
            }

            while !queue.is_empty() {
                let (y_curr, x_curr) = queue.pop_front().unwrap();

                for i in 0..4 {
                    let (y_next, x_next) = (y_curr as i32 + dy[i], x_curr as i32 + dx[i]);

                    if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                        continue;
                    }

                    let (y_next, x_next) = (y_next as usize, x_next as usize);

                    if checked[y_next][x_next] {
                        continue;
                    }

                    if picture[y_next][x_next] == 1 {
                        checked[y_next][x_next] = true;
                        queue.push_back((y_next, x_next));
                    }
                }

                cnt += 1;
            }

            ret_area = ret_area.max(cnt);
        }
    }

    writeln!(out, "{ret_cnt}").unwrap();
    writeln!(out, "{ret_area}").unwrap();
}
