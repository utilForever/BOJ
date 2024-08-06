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

static MOD: i64 = 1_000_000_007;

#[derive(Clone, Debug)]
struct Node {
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }
}

struct FenwickTree {
    data: Vec<Vec<Node>>,
}

impl FenwickTree {
    pub fn new(len_partial: usize, len_total: usize) -> Self {
        Self {
            data: vec![vec![Node::new(0); len_total + 1]; len_partial + 1],
        }
    }

    pub fn update(&mut self, len_partial: usize, len_total: usize, diff: i64) {
        let mut len_total = len_total as i64;

        while len_total < self.data[0].len() as i64 {
            self.data[len_partial][len_total as usize].val = (self.data[len_partial][len_total as usize].val + diff) % MOD;
            len_total += len_total & -len_total;
        }
    }

    pub fn query(&mut self, len_partial: usize, len_total: usize) -> Node {
        if len_partial == 0 {
            return Node::new(1);
        }

        let mut len_total = len_total as i64;
        let mut ret = 0;

        while len_total > 0 {
            ret = (ret + self.data[len_partial][len_total as usize].val) % MOD;
            len_total -= len_total & -len_total;
        }

        Node::new(ret)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), 11);
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<usize>();
    }

    let mut tree = FenwickTree::new(k, n);

    for i in 1..=n {
        for j in 1..=k {
            let ret = tree.query(j - 1, nums[i] - 1);
            tree.update(j, nums[i], ret.val);
        }
    }

    writeln!(out, "{}", tree.query(k, n).val).unwrap();
}
