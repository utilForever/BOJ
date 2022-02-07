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

fn process_dfs(
    vertices: &Vec<Vec<usize>>,
    cut: &mut Vec<bool>,
    discover: &mut Vec<usize>,
    discover_idx: &mut usize,
    start: usize,
    is_root: bool,
) -> usize {
    *discover_idx += 1;
    discover[start] = *discover_idx;

    let mut ret = discover[start];
    let mut child = 0;

    for &next in vertices[start].iter() {
        if discover[next] == 0 {
            child += 1;

            let next_ret = process_dfs(vertices, cut, discover, discover_idx, next, false);
            if !is_root && next_ret >= discover[start] {
                cut[start] = true;
            }
            ret = std::cmp::min(ret, next_ret);
        } else {
            ret = std::cmp::min(ret, discover[next]);
        }
    }

    if is_root && child > 1 {
        cut[start] = true;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (v, e): (usize, usize) = (scan.token(), scan.token());

    let mut vertices = vec![Vec::new(); v + 1];
    let mut cut = vec![false; v + 1];

    for _ in 0..e {
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        vertices[a].push(b);
        vertices[b].push(a);
    }

    let mut discover = vec![0; v + 1];
    let mut discover_idx = 0;

    for i in 1..=v {
        if discover[i] == 0 {
            process_dfs(
                &vertices,
                &mut cut,
                &mut discover,
                &mut discover_idx,
                i,
                true,
            );
        }
    }

    let mut ans = 0;

    for i in 1..=v {
        if cut[i] {
            ans += 1;
        }
    }

    writeln!(out, "{}", ans).unwrap();

    for i in 1..=v {
        if cut[i] {
            write!(out, "{} ", i).unwrap();
        }
    }

    writeln!(out).unwrap();
}
