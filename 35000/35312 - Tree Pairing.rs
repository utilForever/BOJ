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

const MOD: i64 = 998_244_353;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factorial_double = vec![0; 200_001];
    factorial_double[0] = 1;
    factorial_double[1] = 1;

    for i in 2..=200_000 {
        factorial_double[i] = factorial_double[i - 2] * i as i64 % MOD;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut graph = vec![Vec::new(); 2 * n + 1];

        for _ in 0..2 * n - 1 {
            let (u, v, _) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            graph[u].push(v);
            graph[v].push(u);
        }

        let root = 1;
        let mut parent = vec![usize::MAX; 2 * n + 1];

        parent[root] = root;

        let mut stack = Vec::new();
        let mut order = Vec::new();

        stack.push(root);

        while let Some(u) = stack.pop() {
            order.push(u);

            for &v in graph[u].iter() {
                if parent[u] == v {
                    continue;
                }

                if parent[v] == usize::MAX {
                    parent[v] = u;
                    stack.push(v);
                }
            }
        }

        let mut subtree = vec![1; 2 * n + 1];

        for &u in order.iter().rev() {
            if u == root {
                continue;
            }

            subtree[parent[u]] += subtree[u];
        }

        let mut degree_odd = vec![0; 2 * n + 1];

        for &u in order.iter().rev() {
            if u == root {
                continue;
            }

            if subtree[u] % 2 == 1 {
                degree_odd[u] += 1;
                degree_odd[parent[u]] += 1;
            }
        }

        let mut ret = 1;

        for degree in degree_odd {
            ret = (ret * factorial_double[degree]) % MOD;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
