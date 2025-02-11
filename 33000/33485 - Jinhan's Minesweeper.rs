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

const DY: [i64; 4] = [1, 0, -1, 0];
const DX: [i64; 4] = [0, -1, 0, 1];

fn calculate_score(grid: &Vec<Vec<char>>, n: usize, m: usize) -> i64 {
    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                ret += 1;
                continue;
            }

            let mut has_sharp = true;

            for d in 0..4 {
                let y_next = i as i64 + DY[d];
                let x_next = j as i64 + DX[d];

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                    continue;
                }

                if grid[y_next as usize][x_next as usize] == '#' {
                    has_sharp = false;
                    break;
                }
            }

            if has_sharp {
                return 1 << 30;
            }
        }
    }

    ret
}

fn generate_grid(n: usize, m: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['#'; m]; n];
    let mut best = grid.clone();
    let mut score_best = calculate_score(&best, n, m);

    fn record(
        grid: &mut Vec<Vec<char>>,
        best: &mut Vec<Vec<char>>,
        score_best: &mut i64,
        y: usize,
        x: usize,
        n: usize,
        m: usize,
    ) {
        if y == n {
            let score_curr = calculate_score(grid, n, m);

            if score_curr < *score_best {
                *score_best = score_curr;
                *best = grid.clone();
            }

            return;
        }

        let (y_next, x_next) = if x + 1 == m { (y + 1, 0) } else { (y, x + 1) };

        grid[y][x] = '.';
        record(grid, best, score_best, y_next, x_next, n, m);

        grid[y][x] = '#';
        record(grid, best, score_best, y_next, x_next, n, m);
    }

    record(&mut grid, &mut best, &mut score_best, 0, 0, n, m);

    best
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut db = vec![vec![Vec::new(); 5]; 5];

    for i in 1..=4 {
        for j in 1..=4 {
            db[i][j] = generate_grid(i, j);
        }
    }

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let ret = if n.max(m) > 4 {
            vec![vec!['.'; m]; n]
        } else {
            db[n][m].clone()
        };

        writeln!(out, "{}", calculate_score(&ret, n, m)).unwrap();

        for i in 0..n {
            for j in 0..m {
                write!(out, "{}", ret[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
