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

#[derive(Default)]
struct Node {
    items: Vec<usize>,
}

impl Node {
    #[inline]
    fn push(&mut self, val: usize) {
        self.items.push(val);
    }

    #[inline]
    fn drain_into(&mut self, vals: &mut Vec<usize>) {
        if !self.items.is_empty() {
            vals.append(&mut self.items);
        }
    }
}

struct SegmentTree {
    size: usize,
    nodes: Vec<Node>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1usize;

        while size < n {
            size <<= 1;
        }

        let nodes = (0..(2 * size)).map(|_| Node::default()).collect::<Vec<_>>();

        Self { size, nodes }
    }

    #[inline]
    fn add_range(&mut self, left: usize, right: usize, idx: usize) {
        let mut idx_left = left + self.size - 1;
        let mut idx_right = right + self.size - 1;

        loop {
            if (idx_left & 1) == 1 {
                self.nodes[idx_left].push(idx);
                idx_left += 1;
            }

            if (idx_right & 1) == 0 {
                self.nodes[idx_right].push(idx);
                idx_right -= 1;
            }

            if idx_left > idx_right {
                break;
            }

            idx_left >>= 1;
            idx_right >>= 1;
        }
    }

    #[inline]
    fn drain_path(&mut self, pos: usize, vals: &mut Vec<usize>) {
        let mut idx = pos + self.size - 1;

        while idx > 0 {
            self.nodes[idx].drain_into(vals);
            idx >>= 1;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tree = SegmentTree::new(n);

    for i in 1..=n - 1 {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        tree.add_range(l, r, i);
    }

    let mut queue = VecDeque::new();
    queue.push_back(n);

    let mut visited = vec![false; n + 1];
    let mut parent = vec![0; n + 1];
    let mut bucket = Vec::new();

    while let Some(v) = queue.pop_front() {
        tree.drain_path(v, &mut bucket);

        for i in bucket.drain(..) {
            if !visited[i] {
                visited[i] = true;
                parent[i] = v;
                queue.push_back(i);
            }
        }
    }

    if visited[1..=n - 1].iter().any(|&x| !x) {
        writeln!(out, "NO").unwrap();
        return;
    }

    writeln!(out, "YES").unwrap();

    for i in 1..=n - 1 {
        write!(out, "{} ", parent[i]).unwrap();
    }

    writeln!(out).unwrap();
}
