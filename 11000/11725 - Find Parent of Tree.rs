use io::Write;
use std::{collections::VecDeque, io, str};

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

fn process_bfs(tree: &Vec<Vec<usize>>, check: &mut Vec<bool>, parent: &mut Vec<usize>) {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    check[1] = true;
    parent[1] = 0;

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();

        for val in tree[vertex].iter() {
            if !check[*val] {
                check[*val] = true;
                parent[*val] = vertex;
                queue.push_back(*val);
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n: usize = scan.token::<usize>();
    let mut tree = vec![Vec::new(); n + 1];
    let mut check = vec![false; n + 1];
    let mut parent = vec![0; n + 1];

    for _ in 1..n {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        tree[u].push(v);
        tree[v].push(u);
    }

    process_bfs(&tree, &mut check, &mut parent);

    for i in 2..=n {
        writeln!(out, "{}", parent[i]).unwrap();
    }
}
