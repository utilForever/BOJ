use io::Write;
use std::{
    collections::{BTreeSet, VecDeque},
    io, str,
};

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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];
    let mut degrees = vec![0; n + 1];

    for _ in 0..m {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

        graph[x].push(y);
        degrees[y] += 1;
    }

    let mut queue = VecDeque::new();
    let mut vertices = vec![BTreeSet::new(); n + 1];
    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        if degrees[i] == 0 {
            queue.push_back(i);
            vertices[i].insert(i);
            dp[i] = 1;
        }
    }

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        for &next in &graph[curr] {
            for val in vertices[curr].clone().iter() {
                vertices[next].insert(*val);
            }

            degrees[next] -= 1;

            if degrees[next] == 0 {
                queue.push_back(next);

                for val in vertices[next].iter() {
                    dp[next] = (dp[next] + dp[*val]) % MOD;
                }

                dp[next] = (dp[next] + 1) % MOD;
                vertices[next].insert(next);
            }
        }
    }

    let mut ret = 0;

    for i in 1..=n {
        ret = (ret + dp[i]) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
