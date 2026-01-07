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

const MOD: i64 = 1_000_000_007;

fn pow(mut base: i64, mut exp: i64) -> i64 {
    let mut ret = 1;

    base %= MOD;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = ret * base % MOD;
        }

        base = base * base % MOD;
        exp >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    if m != n - 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut factorial = vec![1; n + 1];

    for i in 1..=n {
        factorial[i] = (factorial[i - 1] * i as i64) % MOD;
    }

    let mut parent = vec![0; n + 1];
    let mut order = Vec::with_capacity(n);
    let mut stack = Vec::new();

    stack.push(1);

    while let Some(node) = stack.pop() {
        order.push(node);

        for &next in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            parent[next] = node;
            stack.push(next);
        }
    }

    let mut size_subtree = vec![0; n + 1];

    for &node in order.iter().rev() {
        let mut size = 1;

        for &next in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            size += size_subtree[next];
        }

        size_subtree[node] = size;
    }

    let mut dp = vec![0; n + 1];
    let mut val = 1;

    for i in 1..=n {
        val = val * pow(size_subtree[i] as i64, MOD - 2) % MOD;
    }

    dp[1] = val;

    for &node in order.iter() {
        for &next in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            let size = size_subtree[next];
            let rest = n - size_subtree[next];

            let mut val = dp[node] * size as i64 % MOD;
            val = val * pow(rest as i64, MOD - 2) % MOD;

            dp[next] = val;
        }
    }

    let mut ret = 0;

    for i in 1..=n {
        ret = (ret + dp[i]) % MOD;
    }

    writeln!(out, "{}", factorial[n] * ret % MOD).unwrap();
}
