use io::Write;
use std::{io, str, cmp};

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

fn process_dfs(
    vertices: &Vec<Vec<usize>>,
    discover: &mut Vec<usize>,
    discover_idx: &mut usize,
    res: &mut Vec<(usize, usize)>,
    start: usize,
    parent: usize,
) -> usize {
    *discover_idx += 1;
    discover[start] = *discover_idx;

    let mut ret = discover[start];

    for &next in vertices[start].iter() {
        if next == parent {
            continue;
        }

        if discover[next] == 0 {
            let next_ret = process_dfs(vertices, discover, discover_idx, res, next, start);
            if next_ret > discover[start] {
                res.push((cmp::min(start, next), cmp::max(start, next)));
            }
            ret = std::cmp::min(ret, next_ret);
        } else {
            ret = std::cmp::min(ret, discover[next]);
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (v, e): (usize, usize) = (scan.token(), scan.token());

    let mut vertices = vec![Vec::new(); v + 1];

    for _ in 0..e {
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        vertices[a].push(b);
        vertices[b].push(a);
    }

    let mut discover = vec![0; v + 1];
    let mut discover_idx = 0;
    let mut res = Vec::new();

    for i in 1..=v {
        if discover[i] == 0 {
            process_dfs(
                &vertices,
                &mut discover,
                &mut discover_idx,
                &mut res,
                i,
                0,
            );
        }
    }

    res.sort();

    writeln!(out, "{}", res.len()).unwrap();
    for (a, b) in res {
        writeln!(out, "{} {}", a, b).unwrap();
    }
}
