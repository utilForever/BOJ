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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, r) = (scan.token::<usize>(), scan.token::<usize>());
    let mut c = vec![0; n + 1];
    let mut parent = vec![0; n + 1];
    let mut child = vec![Vec::new(); n + 1];
    let mut visited = vec![false; n + 1];
    let mut f = vec![1; n + 1];

    for i in 1..=n {
        c[i] = scan.token::<usize>();
    }

    for _ in 0..n - 1 {
        let (v1, v2) = (scan.token::<usize>(), scan.token::<usize>());

        parent[v2] = v1;
        child[v1].push(v2);
    }

    let mut ret = 0;

    for _ in 0..n - 1 {
        let mut u = 0;
        let mut max_val = std::f64::MIN;

        for i in 1..=n {
            if !visited[i] && i != r && max_val < (c[i] as f64 / f[i] as f64) {
                max_val = c[i] as f64 / f[i] as f64;
                u = i;
            }
        }

        visited[u] = true;

        let mut v = parent[u];

        while v != 0 && visited[v] {
            v = parent[v];
        }

        ret += c[u] * f[v];

        f[v] += f[u];
        c[v] += c[u];
        parent[u] = v;

        for i in 0..child[u].len() {
            let next = child[u][i];
            parent[next] = v;
        }
    }

    ret += c[r];

    writeln!(out, "{}", ret).unwrap();
}
