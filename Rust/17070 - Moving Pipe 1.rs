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

static dx: [usize; 3] = [0, 1, 1];
static dy: [usize; 3] = [1, 0, 1];

fn calculate(house: &Vec<Vec<usize>>, n: usize, x: usize, y: usize, pipe: usize, cnt: &mut usize) {
    if x == n - 1 && y == n - 1 {
        *cnt += 1;
        return;
    }

    for i in 0..3 {
        // Cannot horizontal + vertical or vertical + horizontal
        if i + pipe == 1 {
            continue;
        }

        let next_x = x + dx[i];
        let next_y = y + dy[i];

        // Check bound and wall
        if next_x >= n || next_y >= n || house[next_x][next_y] == 1 {
            continue;
        }

        // Check diagonal
        if i == 2 && (house[x][y + 1] == 1 || house[x + 1][y] == 1) {
            continue;
        }

        calculate(house, n, next_x, next_y, i, cnt);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut house = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            house[i][j] = scan.token::<usize>();
        }
    }

    let mut cnt = 0;
    calculate(&house, n, 0, 1, 0, &mut cnt);

    writeln!(out, "{}", cnt).unwrap();
}
