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

    let (m, n, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut rectangle = vec![vec![0; n]; m];

    for _ in 0..k {
        let (x1, y1, x2, y2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        for i in y1..y2 {
            for j in x1..x2 {
                rectangle[i][j] = 1;
            }
        }
    }

    let mut cnt = 0;
    let mut areas = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if rectangle[i][j] == 1 {
                continue;
            }

            let mut area = 0;
            let mut stack = Vec::new();
            stack.push((i as i64, j as i64));

            while !stack.is_empty() {
                let (x, y) = stack.pop().unwrap();

                if x < 0 || x >= m as i64 || y < 0 || y >= n as i64 {
                    continue;
                }

                let (x, y) = (x as usize, y as usize);

                if rectangle[x][y] == 1 {
                    continue;
                }

                area += 1;
                rectangle[x][y] = 1;

                stack.push((x as i64 - 1, y as i64));
                stack.push((x as i64 + 1, y as i64));
                stack.push((x as i64, y as i64 - 1));
                stack.push((x as i64, y as i64 + 1));
            }

            cnt += 1;
            areas.push(area);
        }
    }

    areas.sort();

    writeln!(out, "{cnt}").unwrap();

    for area in areas {
        write!(out, "{area} ").unwrap();
    }

    writeln!(out).unwrap();
}
