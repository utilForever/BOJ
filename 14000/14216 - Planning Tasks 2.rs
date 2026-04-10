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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_hungarian(costs: &Vec<Vec<i64>>) -> (i64, Vec<usize>) {
    let n = costs.len();

    if n == 0 {
        return (0, Vec::new());
    }

    let m = costs[0].len();
    let mut u = vec![0; n + 1];
    let mut v = vec![0; m + 1];
    let mut matched = vec![0; m + 1];
    let mut path = vec![0; m + 1];

    for i in 1..=n {
        matched[0] = i;

        let mut j0 = 0;
        let mut val_min = vec![i64::MAX / 4; m + 1];
        let mut visited = vec![false; m + 1];

        loop {
            visited[j0] = true;

            let i0 = matched[j0];
            let mut delta = i64::MAX / 4;
            let mut j1 = 0;

            for j in 1..=m {
                if visited[j] {
                    continue;
                }

                let val = costs[i0 - 1][j - 1] - u[i0] - v[j];

                if val < val_min[j] {
                    val_min[j] = val;
                    path[j] = j0;
                }

                if val_min[j] < delta {
                    delta = val_min[j];
                    j1 = j;
                }
            }

            for j in 0..=m {
                if visited[j] {
                    u[matched[j]] += delta;
                    v[j] -= delta;
                } else {
                    val_min[j] -= delta;
                }
            }

            j0 = j1;

            if matched[j0] == 0 {
                break;
            }
        }

        loop {
            let j1 = path[j0];

            matched[j0] = matched[j1];
            j0 = j1;

            if j0 == 0 {
                break;
            }
        }
    }

    let mut assignment = vec![usize::MAX; n];

    for j in 1..=m {
        if matched[j] != 0 {
            assignment[matched[j] - 1] = j - 1;
        }
    }

    let mut cost_min = 0;

    for i in 0..n {
        cost_min += costs[i][assignment[i]];
    }

    (cost_min, assignment)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut costs = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = scan.token::<i64>();
        }
    }

    let (cost_min, _) = process_hungarian(&costs);

    writeln!(out, "{cost_min}").unwrap();
}
