use io::Write;
use std::{io, str, vec};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v, x) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[u].push((v, x));
        graph[v].push((u, x));
    }

    let mut dist = vec![0; n + 1];
    let mut visited = vec![false; n + 1];
    let mut stack = Vec::new();

    dist[1] = 0;
    visited[1] = true;
    stack.push(1);

    while let Some(u) = stack.pop() {
        for &(v, x) in graph[u].iter() {
            if visited[v] {
                continue;
            }

            visited[v] = true;
            dist[v] = dist[u] + x;
            stack.push(v);
        }
    }

    let mut dists = (1..=n).map(|i| dist[i]).collect::<Vec<_>>();
    dists.sort_unstable();

    let mut vals = Vec::new();
    let mut counts = Vec::new();

    for d in dists {
        if vals.last() == Some(&d) {
            let len = counts.len();
            counts[len - 1] += 1;
        } else {
            vals.push(d);
            counts.push(1);
        }
    }

    let mut dp = vec![false; n + 1];
    let mut prev_group = vec![usize::MAX; n + 1];
    let mut prev_sum = vec![usize::MAX; n + 1];

    dp[0] = true;

    for (i, &cnt) in counts.iter().enumerate() {
        for j in (cnt..=n).rev() {
            if !dp[j] && dp[j - cnt] {
                dp[j] = true;
                prev_group[j] = i;
                prev_sum[j] = j - cnt;
            }
        }
    }

    if !dp[m] {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut selected = vec![false; counts.len()];
    let mut idx = m;

    while idx > 0 {
        let group = prev_group[idx];
        let sum = prev_sum[idx];

        if group == usize::MAX || sum == usize::MAX {
            println!("-1");
            return;
        }

        selected[group] = true;
        idx = sum;
    }

    let mut ret = Vec::new();

    for i in 0..counts.len() {
        let val_curr = selected[i] as u8;
        let val_next = if i + 1 < counts.len() {
            selected[i + 1] as u8
        } else {
            0
        };

        if (val_curr ^ val_next) == 1 {
            ret.push(vals[i]);
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
