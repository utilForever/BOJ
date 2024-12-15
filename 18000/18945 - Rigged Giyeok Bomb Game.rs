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
    sum: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { sum: val }
    }

    fn merge(&mut self, other: &Self) -> Node {
        let mut ret = Node::new(0);
        ret.sum = self.sum + other.sum;

        ret
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Node>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Node::new(0); real_n * 4],
        }
    }

    pub fn update(&mut self, index: usize, val: i64) {
        self.update_internal(index, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        index: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if index > node_end || index < node_start {
            return;
        }

        if node_start == node_end {
            self.data[node].sum += val;
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(index, val, node * 2, node_start, mid);
        self.update_internal(index, val, node * 2 + 1, mid + 1, node_end);

        let mut left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();
        self.data[node] = left.merge(&right);
    }

    fn query(&mut self, start: usize, end: usize) -> Node {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Node {
        if end < node_start || node_end < start {
            return Node { sum: 0 };
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(start, end, node * 2, node_start, mid);
        let right = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        let sum = left.sum + right.sum;

        Node { sum }
    }
}

fn analyze_board(board: &Vec<Vec<char>>) -> (usize, i64, bool, i64) {
    let r = board.len();
    let c = board[0].len();

    let mut pos_bomb_a = Vec::with_capacity(r * c);
    let mut pos_bomb_b = Vec::with_capacity(r * c);

    for i in 0..r {
        for j in 0..c {
            match board[i][j] {
                'A' => pos_bomb_a.push((i, j)),
                'B' => pos_bomb_b.push((i, j)),
                _ => {}
            }
        }
    }

    let bomb_only_one = if pos_bomb_b.len() == 1 { 'B' } else { 'A' };
    let pos_bomb_only_one = if bomb_only_one == 'A' {
        pos_bomb_a[0]
    } else {
        pos_bomb_b[0]
    };

    let sign = if bomb_only_one == 'A' { -1 } else { 1 };
    let (mut r_max, mut c_min) = (-1, c);
    let mut p = 0;
    let mut q = 0;

    for i in 0..r {
        for j in 0..c {
            if board[i][j] != bomb_only_one && board[i][j] != 'C' {
                if i >= pos_bomb_only_one.0 && j <= pos_bomb_only_one.1 {
                    p += 1;
                } else {
                    if i > pos_bomb_only_one.0 {
                        c_min = c_min.min(j);
                    } else if j < pos_bomb_only_one.1 {
                        r_max = r_max.max(i as i64);
                    }

                    q += 1;
                }
            }
        }
    }

    let mut is_freebomb = false;
    let r_max = r_max as usize;

    'outer: for i in r_max + 1..=pos_bomb_only_one.0 {
        for j in pos_bomb_only_one.1..c_min {
            if board[i][j] != bomb_only_one && board[i][j] != 'C' {
                is_freebomb = true;
                break 'outer;
            }
        }
    }

    (p, q, is_freebomb, sign)
}

fn compute_advantage(board: &Vec<Vec<char>>) -> i64 {
    let (p, q, is_freebomb, sign) = analyze_board(board);
    let base = q * (1i64 << 40);

    let ret = if is_freebomb {
        let sub = 1i64 << (39 - p);
        base - sub
    } else {
        let sub = 1i64 << (40 - p);
        base - sub
    };

    ret * sign
}

// Reference: https://www.youtube.com/watch?v=ZYj4NkeGPdM
// Reference: https://is.muni.cz/th/325040/fi_b/Combinatorial_games.pdf
// Reference: https://stonejjun.tistory.com/71
// Reference: https://rkm0959.tistory.com/139
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];

    for idx in 1..=n {
        let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
        let mut board = vec![vec![' '; c]; r];

        for i in 0..r {
            let line = scan.token::<String>();

            for (j, ch) in line.chars().enumerate() {
                board[i][j] = ch;
            }
        }

        nums[idx] = compute_advantage(&board);
    }

    let mut tree = SegmentTree::new(n);

    for i in 1..=n {
        tree.update(i, nums[i]);
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (k, u, v) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        tree.update(k, -2 * nums[k]);
        nums[k] *= -1;

        writeln!(
            out,
            "{}",
            if tree.query(u, v).sum > 0 {
                "Ahgus"
            } else {
                "Bagus"
            }
        )
        .unwrap();
    }
}
