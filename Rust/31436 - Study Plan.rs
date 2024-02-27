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

static MOD: i64 = 998_244_353;

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &Vec<bool>,
    b: &Vec<i64>,
    values: &mut Vec<i64>,
    idx1: usize,
) -> i64 {
    let mut ret = b[idx1];

    for &idx2 in &graph[idx1] {
        if check[idx2] {
            continue;
        }

        ret += process_dfs(graph, check, b, values, idx2);
    }

    if !check[idx1] {
        values.push(ret);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, x, y) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        a[i] = scan.token::<i64>();
        graph[a[i] as usize].push(i);
    }

    for i in 1..=n {
        b[i] = scan.token::<i64>();
    }

    let mut values = Vec::new();
    let mut visited = vec![0; n + 1];
    let mut check = vec![false; n + 1];
    let mut num = 0;

    for i in 1..=n {
        if visited[i] != 0 {
            continue;
        }

        num += 1;

        let mut idx1 = i;
        let mut idx2 = 0;

        loop {
            visited[idx1] = num;

            if visited[a[idx1] as usize] != 0 {
                if visited[a[idx1] as usize] == num {
                    idx2 = a[idx1] as usize;
                }

                break;
            }

            idx1 = a[idx1] as usize;
        }

        if idx2 == 0 {
            continue;
        }

        idx1 = idx2;

        loop {
            check[idx1] = true;
            idx1 = a[idx1] as usize;

            if idx1 == idx2 {
                break;
            }
        }

        idx1 = idx2;
        let mut sum = 0;

        loop {
            sum += process_dfs(&graph, &check, &b, &mut values, idx1);
            idx1 = a[idx1] as usize;

            if idx1 == idx2 {
                break;
            }
        }

        values.push(sum);
    }

    let mut dp = vec![0; y as usize + 1];
    dp[0] = 1;

    for value in values {
        let value = value as usize;

        for i in value..=y as usize {
            dp[i] = (dp[i] + dp[i - value]) % MOD;
        }
    }

    let mut ret = 0;

    for i in x..=y {
        ret = (ret + dp[i as usize]) % MOD;
    }

    writeln!(out, "{ret}").unwrap()
}
