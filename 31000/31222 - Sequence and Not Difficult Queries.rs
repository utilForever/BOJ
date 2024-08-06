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

#[derive(Clone, Debug)]
struct Node {
    val_left: i64,
    val_right: i64,
    count: i64,
}

impl Node {
    fn new(val_left: i64, val_right: i64, count: i64) -> Self {
        Self {
            val_left,
            val_right,
            count,
        }
    }

    fn merge(&mut self, other: &Self) -> Node {
        if self.val_right == other.val_left {
            Node {
                val_left: self.val_left,
                val_right: other.val_right,
                count: self.count + other.count - 1,
            }
        } else {
            Node {
                val_left: self.val_left,
                val_right: other.val_right,
                count: self.count + other.count,
            }
        }
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        Self {
            size: n,
            data: vec![Node::new(0, 0, 0); n * 2],
        }
    }

    fn set(&mut self, index: usize, val: Node) {
        self.data[index + self.size] = val;
    }

    fn init(&mut self) {
        for i in (0..self.size).rev() {
            let mut left = self.data[i * 2].clone();
            let right = self.data[i * 2 + 1].clone();

            self.data[i] = left.merge(&right);
        }
    }

    pub fn update(&mut self, mut index: usize, val: Node) {
        index += self.size;
        self.data[index] = val;

        while index > 1 {
            let mut left = self.data[index].clone();
            let mut right = self.data[index ^ 1].clone();

            self.data[index / 2] = if index % 2 == 0 {
                left.merge(&right)
            } else {
                right.merge(&left)
            };

            index /= 2;
        }
    }

    fn query(&mut self, mut start: usize, mut end: usize) -> Node {
        let mut left = Node::new(0, 0, 0);
        let mut right = Node::new(0, 0, 0);

        start += self.size;
        end += self.size;

        while start <= end {
            if start % 2 == 1 {
                left = left.merge(&self.data[start]);
                start += 1;
            }

            if end % 2 == 0 {
                right = self.data[end].merge(&right);
                end -= 1;
            }

            start /= 2;
            end /= 2;
        }

        left.merge(&right)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut tree = SegmentTree::new(n);
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
        tree.set(i, Node::new(nums[i], nums[i], 1));
    }

    tree.init();

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, x) = (scan.token::<usize>(), scan.token::<i64>());
            tree.update(i - 1, Node::new(x, x, 1));
        } else {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            let ret = tree.query(l - 1, r - 1);

            writeln!(out, "{}", ret.count).unwrap();
        }
    }
}
