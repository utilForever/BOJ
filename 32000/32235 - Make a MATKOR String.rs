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

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, y: i64) -> i64 {
    if y == 0 {
        1
    } else if y == 1 {
        x
    } else if y % 2 == 1 {
        pow(x * x % MOD, y / 2) * x % MOD
    } else {
        pow(x * x % MOD, y / 2)
    }
}

#[derive(Default, Clone, Copy)]
struct SegNode {
    val: [[[i64; 3]; 7]; 7],
}

impl std::ops::Add for SegNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = SegNode::default();

        for i in 0..7 {
            for j1 in 0..7 - i {
                for j2 in 0..7 - i - j1 {
                    ret.val[i][j1 + j2][0] += self.val[i][j1][0] * rhs.val[i + j1][j2][0];
                    ret.val[i][j1 + j2][0] %= MOD;

                    ret.val[i][j1 + j2][1] += self.val[i][j1][0] * rhs.val[i + j1][j2][1];
                    ret.val[i][j1 + j2][1] += self.val[i][j1][1] * rhs.val[i + j1][j2][0];
                    ret.val[i][j1 + j2][1] %= MOD;

                    ret.val[i][j1 + j2][2] += 2 * self.val[i][j1][1] * rhs.val[i + j1][j2][1];
                    ret.val[i][j1 + j2][2] += self.val[i][j1][0] * rhs.val[i + j1][j2][2];
                    ret.val[i][j1 + j2][2] += self.val[i][j1][2] * rhs.val[i + j1][j2][0];
                    ret.val[i][j1 + j2][2] %= MOD;
                }
            }
        }

        ret
    }
}

const MATKOR: [char; 6] = ['M', 'A', 'T', 'K', 'O', 'R'];

struct SegmentTree {
    nodes: Vec<SegNode>,
    data: Vec<char>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![SegNode::default(); 1 << 18],
            data: vec![' '; n],
        }
    }

    fn init(&mut self, node: usize, left: usize, right: usize) {
        if left == right {
            for i in 0..7 {
                self.nodes[node].val[i][0][0] = 1;
                self.nodes[node].val[i][0][1] = 1;
                self.nodes[node].val[i][0][2] = 1;

                if i == 6 {
                    continue;
                }

                if self.data[left] <= MATKOR[i] {
                    self.nodes[node].val[i][1][0] = 1;
                    self.nodes[node].val[i][1][1] =
                        (MATKOR[i] as u8 - self.data[left] as u8) as i64;
                    self.nodes[node].val[i][1][2] =
                        self.nodes[node].val[i][1][1] * self.nodes[node].val[i][1][1];
                }
            }
        } else {
            let mid = (left + right) / 2;

            self.init(node * 2, left, mid);
            self.init(node * 2 + 1, mid + 1, right);

            self.nodes[node] = self.nodes[node * 2] + self.nodes[node * 2 + 1];
        }
    }

    fn query(&self, node: usize, left: usize, right: usize, start: usize, end: usize) -> SegNode {
        if right < start || end < left {
            let mut initial = SegNode::default();

            for i in 0..7 {
                initial.val[i][0][0] = 1;
                initial.val[i][0][1] = 1;
                initial.val[i][0][2] = 1;
            }

            return initial;
        }

        if start <= left && right <= end {
            return self.nodes[node];
        }

        let mid = (left + right) / 2;

        self.query(node * 2, left, mid, start, end)
            + self.query(node * 2 + 1, mid + 1, right, start, end)
    }

    fn update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: char) {
        if right < idx || idx < left {
            return;
        }

        if left == idx && idx == right {
            for i in 0..7 {
                self.nodes[node].val[i][0][0] = 1;
                self.nodes[node].val[i][0][1] = 1;
                self.nodes[node].val[i][0][2] = 1;

                if i == 6 {
                    continue;
                }

                if val <= MATKOR[i] {
                    self.nodes[node].val[i][1][0] = 1;
                    self.nodes[node].val[i][1][1] = (MATKOR[i] as u8 - val as u8) as i64;
                    self.nodes[node].val[i][1][2] =
                        self.nodes[node].val[i][1][1] * self.nodes[node].val[i][1][1];
                } else {
                    self.nodes[node].val[i][1][0] = 0;
                    self.nodes[node].val[i][1][1] = 0;
                    self.nodes[node].val[i][1][2] = 0;
                }
            }

            return;
        }

        let mid = (left + right) / 2;

        self.update(node * 2, left, mid, idx, val);
        self.update(node * 2 + 1, mid + 1, right, idx, val);

        self.nodes[node] = self.nodes[node * 2] + self.nodes[node * 2 + 1];
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = s.len();

    let mut segment_tree = SegmentTree::new(n);
    segment_tree.data = s;

    segment_tree.init(1, 0, n - 1);

    let q = scan.token::<i64>();

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, c) = (scan.token::<usize>(), scan.token::<char>());
            segment_tree.update(1, 0, n - 1, i - 1, c);
        } else {
            let (i, j) = (scan.token::<usize>(), scan.token::<usize>());
            let ret = segment_tree.query(1, 0, n - 1, i - 1, j - 1);

            write!(out, "{}", ret.val[0][6][0]).unwrap();

            if ret.val[0][6][0] == 0 {
                writeln!(out).unwrap();
                continue;
            }

            let cnt_inv = pow(ret.val[0][6][0], MOD - 2);
            let ret1 = ret.val[0][6][2] * cnt_inv % MOD;

            let mut ret2 = ret.val[0][6][1] * cnt_inv % MOD;
            ret2 = (ret2 * ret2) % MOD;

            writeln!(out, " {}", (ret1 - ret2 + MOD) % MOD).unwrap();
        }
    }
}
