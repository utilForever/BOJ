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

fn process_dfs(
    dists: &Vec<Vec<f64>>,
    visited: &mut Vec<bool>,
    matched: &mut Vec<Option<usize>>,
    limit: f64,
    u: usize,
) -> bool {
    for v in 0..dists[u].len() {
        if visited[v] || dists[u][v] > limit {
            continue;
        }

        visited[v] = true;

        if matched[v].is_none() || process_dfs(dists, visited, matched, limit, matched[v].unwrap())
        {
            matched[v] = Some(u);
            return true;
        }
    }

    false
}

fn check(dists: &Vec<Vec<f64>>, limit: f64) -> bool {
    let n = dists.len();
    let mut matched = vec![None; n];

    for u in 0..n {
        let mut visited = vec![false; n];

        if !process_dfs(dists, &mut visited, &mut matched, limit, u) {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, t) = (scan.token::<usize>(), scan.token::<f64>());
        let mut locations_first = vec![(0.0, 0.0); n];
        let mut locations_second = vec![(0.0, 0.0); n];

        for i in 0..n {
            locations_first[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        for i in 0..n {
            locations_second[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut dists = vec![vec![0.0; n]; n];
        let mut dists_all = Vec::with_capacity(n * n);

        for i in 0..n {
            for j in 0..n {
                let dist_border_first = locations_first[i]
                    .0
                    .min(1.0 - locations_first[i].0)
                    .min(locations_first[i].1)
                    .min(1.0 - locations_first[i].1);
                let dist_border_second = locations_second[j]
                    .0
                    .min(1.0 - locations_second[j].0)
                    .min(locations_second[j].1)
                    .min(1.0 - locations_second[j].1);

                let dx = locations_first[i].0 - locations_second[j].0;
                let dy = locations_first[i].1 - locations_second[j].1;

                dists[i][j] =
                    dist_border_first + dist_border_second + 2.0 * (dx * dx + dy * dy).sqrt();
                dists_all.push(dists[i][j]);
            }
        }

        dists_all.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut left = 0;
        let mut right = dists_all.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if check(&dists, dists_all[mid]) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        writeln!(out, "Data Set {i}:").unwrap();
        writeln!(out, "{:.2}", dists_all[left] / t).unwrap();
        writeln!(out).unwrap();
    }
}
