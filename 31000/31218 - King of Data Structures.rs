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

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![false; m]; n];
    let mut cnt_grasses = n * m;

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (dy, dx, mut y, mut x) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>() - 1,
                scan.token::<i64>() - 1,
            );

            loop {
                if grid[y as usize][x as usize] {
                    break;
                }

                grid[y as usize][x as usize] = true;
                cnt_grasses -= 1;

                if y + dy < 0 || y + dy >= n as i64 || x + dx < 0 || x + dx >= m as i64 {
                    break;
                }

                y += dy;
                x += dx;
            }
        } else if command == 2 {
            let (y, x) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(
                out,
                "{}",
                match grid[y - 1][x - 1] {
                    true => 1,
                    false => 0,
                }
            )
            .unwrap();
        } else {
            writeln!(out, "{cnt_grasses}").unwrap();
        }
    }
}
