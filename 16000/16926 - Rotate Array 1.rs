use io::Write;
use std::{cmp, io, str};

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

fn rotate_array(arr: &mut Vec<Vec<i64>>, n: usize, m: usize) {
    let dx: [i64; 4] = [0, 1, 0, -1];
    let dy: [i64; 4] = [1, 0, -1, 0];
    let num_rotate = cmp::min(n, m) / 2;

    for i in 0..num_rotate {
        let (mut x, mut y) = (i, i);
        let val = arr[x][y];
        let mut idx = 0;

        while idx < 4 {
            let next_x = x as i64 + dx[idx];
            let next_y = y as i64 + dy[idx];

            if next_x < i as i64 || next_x >= (n - i) as i64 || next_y < i as i64 || next_y >= (m - i) as i64 {
                idx += 1;
                continue;
            }

            arr[x][y] = arr[next_x as usize][next_y as usize];

            x = next_x as usize;
            y = next_y as usize;
        }

        arr[i + 1][i] = val;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut arr = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            arr[i][j] = scan.token::<i64>();
        }
    }

    for _ in 0..r {
        rotate_array(&mut arr, n, m);
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{} ", arr[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
