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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    matched: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched[next] == -1 || process_dfs(graph, check, matched, matched[next] as usize) {
            matched[next] = idx as i64;
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut idxes_female = vec![-1; n + 1];
    let mut idxes_male = vec![-1; n + 1];
    let mut idx_female = 0;
    let mut idx_male = 0;

    for idx in 1..=n {
        if idx % 2 == 0 {
            idxes_male[idx] = idx_male;
            idx_male += 1;
        } else {
            idxes_female[idx] = idx_female;
            idx_female += 1;
        }
    }

    let mut graph = vec![Vec::new(); idx_female as usize];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        if u % 2 == 1 && v % 2 == 0 {
            let idx = idxes_female[u] as usize;
            let val = idxes_male[v] as usize;
            graph[idx].push(val);
        } else if u % 2 == 0 && v % 2 == 1 {
            let idx = idxes_female[v] as usize;
            let val = idxes_male[u] as usize;
            graph[idx].push(val);
        }
    }

    let mut check = vec![false; idx_male as usize];
    let mut matched = vec![-1; idx_male as usize];
    let mut val = 0;

    for i in 0..idx_female as usize {
        check.fill(false);

        if process_dfs(&graph, &mut check, &mut matched, i) {
            val += 1;
        }
    }

    let mut ret = 2 * val;

    if n > ret {
        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
