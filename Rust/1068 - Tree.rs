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

fn process_dfs(nodes: &Vec<Vec<usize>>, idx_to_delete: usize, ret: &mut i64, idx: usize) {
    if idx == idx_to_delete {
        return;
    }

    if nodes[idx].is_empty() {
        *ret += 1;
        return;
    }

    for &next_idx in nodes[idx].iter() {
        if nodes[idx].len() == 1 && next_idx == idx_to_delete {
            *ret += 1;
            continue;
        }

        process_dfs(nodes, idx_to_delete, ret, next_idx);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nodes = vec![Vec::new(); n];
    let mut idx_root = 0;

    for i in 0..n {
        let num = scan.token::<i64>();

        if num == -1 {
            idx_root = i;
            continue;
        }

        nodes[num as usize].push(i);
    }

    let idx_to_delete = scan.token::<usize>();
    let mut ret = 0;

    process_dfs(&nodes, idx_to_delete, &mut ret, idx_root);

    writeln!(out, "{ret}").unwrap();
}
