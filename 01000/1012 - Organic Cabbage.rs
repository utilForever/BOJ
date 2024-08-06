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

fn process_dfs(field: &Vec<Vec<i64>>, checked: &mut Vec<Vec<bool>>, x: usize, y: usize, m: usize, n: usize) {
    if x >= m || y >= n || checked[y][x] {
        return;
    }

    checked[y][x] = true;

    if field[y][x] == 1 {
        process_dfs(field, checked, x - 1, y, m, n);
        process_dfs(field, checked, x + 1, y, m, n);
        process_dfs(field, checked, x, y - 1, m, n);
        process_dfs(field, checked, x, y + 1, m, n);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (m, n, k) = (scan.token::<usize>(), scan.token::<usize>(), scan.token::<usize>());
        let mut field = vec![vec![0; m]; n];
        let mut checked = vec![vec![false; m]; n];

        for _ in 0..k {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
            field[y][x] = 1;
        }

        let mut cnt = 0;

        for i in 0..n {
            for j in 0..m {
                if field[i][j] == 1 && !checked[i][j] {
                    process_dfs(&field, &mut checked, j, i, m, n);
                    cnt += 1;
                }
            }
        }

        writeln!(out, "{}", cnt).unwrap();
    }
}
