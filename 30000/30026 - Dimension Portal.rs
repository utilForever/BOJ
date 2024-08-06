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

fn process_union(parent: &mut Vec<usize>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    parent[a] = b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut parent = vec![0; n + 1];
    let mut nums = vec![0; n + 1];
    let mut nums_inv = vec![0; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    for i in 1..=n {
        let num = scan.token::<usize>();
        nums[i] = num;
        nums_inv[nums[i]] = i;

        process_union(&mut parent, i, num);
    }

    let mut ret = -1;

    for i in 1..=n {
        if find(&mut parent, i) == i {
            ret += 1;
        }
    }

    writeln!(out, "{ret} {ret}").unwrap();

    for i in 1..n {
        if find(&mut parent, i) != find(&mut parent, i + 1) {
            writeln!(out, "{} {}", nums_inv[i], nums_inv[i + 1]).unwrap();

            nums_inv.swap(i, i + 1);
            process_union(&mut parent, i, i + 1);
        }
    }
}
