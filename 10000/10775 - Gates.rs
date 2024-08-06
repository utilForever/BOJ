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

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (g, p) = (scan.token::<usize>(), scan.token::<usize>());
    let mut planes = vec![0; p + 1];

    for i in 1..=p {
        planes[i] = scan.token::<usize>();
    }

    let mut parent = vec![0; g + 1];

    for i in 1..=g {
        parent[i] = i;
    }

    let mut ret = 0;

    for plane in planes.iter().skip(1) {
        let idx_gate = find(&mut parent, *plane);
        if idx_gate == 0 {
            break;
        }

        ret += 1;

        let idx_gate_next = parent[idx_gate - 1];
        parent[idx_gate] = find(&mut parent, idx_gate_next);
    }

    writeln!(out, "{ret}").unwrap();
}
