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

fn process_dfs(
    map: &Vec<Vec<i64>>,
    dp: &mut Vec<Vec<i64>>,
    (y, x): (usize, usize),
    (m, n): (usize, usize),
) -> i64 {
    if y == m - 1 && x == n - 1 {
        return 1;
    }

    if dp[y][x] != -1 {
        return dp[y][x];
    }

    dp[y][x] = 0;

    let dy = vec![-1, 1, 0, 0];
    let dx = vec![0, 0, -1, 1];

    for i in 0..4 {
        let y_next = y as i64 + dy[i];
        let x_next = x as i64 + dx[i];

        if y_next < 0 || y_next >= m as i64 || x_next < 0 || x_next >= n as i64 {
            continue;
        }

        let (y_next, x_next) = (y_next as usize, x_next as usize);

        if map[y_next][x_next] < map[y][x] {
            dp[y][x] += process_dfs(map, dp, (y_next, x_next), (m, n));
        }
    }

    dp[y][x]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            map[i][j] = scan.token::<i64>();
        }
    }

    let mut dp = vec![vec![-1; n]; m];

    writeln!(out, "{}", process_dfs(&map, &mut dp, (0, 0), (m, n))).unwrap();
}
