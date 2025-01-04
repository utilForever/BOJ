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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut seeds = vec![(0, 0, Vec::new()); m];

        for i in 0..m {
            let (t1, t2, a) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );
            let mut pieces = vec![0; a];

            for j in 0..a {
                let q = scan.token::<usize>();
                pieces[j] = q - 1;
            }

            seeds[i] = (t1, t2, pieces);
        }

        let mut available = vec![Vec::new(); 101];

        for (t1, t2, pieces) in seeds.iter() {
            for t in *t1..*t2 {
                for &piece in pieces.iter() {
                    available[t].push(piece);
                }
            }
        }

        let mut ret = -1;

        for t in 1..=100 {
            let mut graph = vec![Vec::new(); n];

            for idx in 0..n {
                for slot in 0..t {
                    if slot <= 100 {
                        if available[slot].contains(&idx) {
                            graph[idx].push(slot);
                        }
                    }
                }
            }

            let mut check = vec![false; t];
            let mut matched = vec![-1; t];
            let mut val = 0;

            for i in 0..n {
                check.fill(false);

                if process_dfs(&graph, &mut check, &mut matched, i) {
                    val += 1;
                }
            }

            if val == n {
                ret = t as i64;
                break;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
