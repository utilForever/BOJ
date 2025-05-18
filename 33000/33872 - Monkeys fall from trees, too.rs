use io::Write;
use std::{collections::HashMap, io, str};

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

    let (n, h) = (scan.token::<usize>(), scan.token::<i64>());
    let (s, e) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    let mut bananas = vec![0; n];

    for i in 0..n {
        bananas[i] = scan.token::<i64>();
    }

    let mut hawks = vec![false; n];

    for i in 0..n {
        hawks[i] = scan.token::<i64>() == 1;
    }

    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dp = HashMap::new();
    dp.insert((s, 1 << s, 0), bananas[s]);

    for _ in 0..h {
        let mut dp_new = HashMap::new();

        for (&(node, once, twice), &val) in dp.iter() {
            for &next in graph[node].iter() {
                if hawks[next] || (twice & 1 << next != 0) {
                    continue;
                }

                let (next_once, next_twice, next_gain) = if once & 1 << next == 0 {
                    (once | 1 << next, twice, bananas[next])
                } else {
                    (once ^ (1 << next), twice | 1 << next, 0)
                };

                let entry = dp_new
                    .entry((next, next_once, next_twice))
                    .or_insert(val + next_gain);
                *entry = (*entry).max(val + next_gain);
            }
        }

        dp = dp_new;
    }

    let mut ret = -1;

    for (&(node, _, _), &val) in dp.iter() {
        if node == e {
            ret = ret.max(val);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
