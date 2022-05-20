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

fn find(pos: &mut Vec<i64>, idx: usize) -> i64 {
    if pos[idx] < 0 {
        idx as i64
    } else {
        pos[idx] = find(pos, pos[idx] as usize);
        pos[idx]
    }
}

fn merge(pos: &mut Vec<i64>, p: usize, q: usize) {
    let mut idx_p = find(pos, p);
    let mut idx_q = find(pos, q);

    if idx_p != idx_q {
        if pos[idx_p as usize] > pos[idx_q as usize] {
            std::mem::swap(&mut idx_p, &mut idx_q);
        }

        pos[idx_p as usize] += pos[idx_q as usize];
        pos[idx_q as usize] = idx_p;
    }
}

// Reference: https://rebro.kr/153
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut edges = vec![(0, 0, 0); m];

    for i in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        edges[i] = (a, b, c);
    }

    edges.sort_by(|&(_, _, a), &(_, _, b)| a.cmp(&b));

    let q = scan.token::<usize>();
    let mut queries = vec![(0, 0); q];

    for i in 0..q {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        queries[i] = (x, y);
    }

    let mut left = vec![0; q];
    let mut right = vec![0; q];

    for i in 0..q {
        left[i] = 1;
        right[i] = m;
    }

    let mut queries_mid = vec![Vec::new(); m + 1];
    let mut ans = vec![(0, 0); q];

    loop {
        for i in 1..=m {
            queries_mid[i].clear();
        }

        let mut should_check = false;

        for i in 0..q {
            if left[i] <= right[i] {
                should_check = true;
                queries_mid[(left[i] + right[i]) / 2].push(i);
            }
        }

        if !should_check {
            break;
        }

        let mut pos = vec![-1; n + 1];
        let mut idx = 1;

        for edge in edges.iter() {
            merge(&mut pos, edge.0, edge.1);

            for mid in queries_mid[idx].iter() {
                if find(&mut pos, queries[*mid].0) == find(&mut pos, queries[*mid].1) {
                    let ret = find(&mut pos, queries[*mid].0);
                    
                    ans[*mid] = (edge.2, pos[ret as usize].abs());
                    right[*mid] = idx - 1;
                } else {
                    left[*mid] = idx + 1;
                }
            }

            idx += 1;
        }
    }

    for i in 0..q {
        if left[i] > m {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{} {}", ans[i].0, ans[i].1).unwrap();
        }
    }
}
