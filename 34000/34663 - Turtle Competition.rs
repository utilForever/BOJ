use io::Write;
use std::{collections::BTreeSet, io, str};

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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }

    fn query_range(&self, left: usize, right: usize) -> i64 {
        self.query(right) - self.query(left - 1)
    }
}

fn obstacle_prev(set: &BTreeSet<usize>, x: usize) -> usize {
    *set.range(..x).next_back().unwrap()
}

fn obstacle_next(set: &BTreeSet<usize>, x: usize, n: usize) -> usize {
    *set.range((x + 1)..=n + 1).next().unwrap()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut grid = vec![vec![' '; n]; 2];

    for i in 0..2 {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mut is_empty = vec![vec![false; n + 2]; 2];
    let mut obstacles = vec![BTreeSet::new(); 2];

    for i in 0..2 {
        obstacles[i].insert(0);
        obstacles[i].insert(n + 1);
    }

    for i in 0..2 {
        for j in 0..n {
            if grid[i][j] == '#' {
                obstacles[i].insert(j + 1);
            } else {
                is_empty[i][j + 1] = true;
            }
        }
    }

    let mut bridge = vec![false; n + 2];
    let mut tree = FenwickTree::new(n);

    for i in 1..=n {
        bridge[i] = is_empty[0][i] && is_empty[1][i];

        if !bridge[i] {
            tree.update(i, 1);
        }
    }

    for _ in 0..q {
        let (op, y, x) = (
            scan.token::<i64>(),
            scan.token::<usize>() - 1,
            scan.token::<usize>(),
        );

        if op == 1 {
            if is_empty[y][x] {
                is_empty[y][x] = false;
                obstacles[y].insert(x);
            } else {
                is_empty[y][x] = true;
                obstacles[y].remove(&x);
            }

            let bridge_prev = bridge[x];
            let bridge_next = is_empty[0][x] && is_empty[1][x];

            if bridge_prev != bridge_next {
                bridge[x] = bridge_next;

                if bridge_next {
                    tree.update(x, -1);
                } else {
                    tree.update(x, 1);
                }
            }
        } else {
            let obstacle_left = obstacle_prev(&obstacles[y], x);
            let obstacle_right = obstacle_next(&obstacles[y], x, n);

            let cnt_non_bridge_total = tree.query_range(obstacle_left + 1, obstacle_right - 1);
            let cnt_non_bridge_x = if bridge[x] { 0 } else { 1 };
            let has_another_non_bridge = cnt_non_bridge_total - cnt_non_bridge_x > 0;

            let mut can_win_vertical = false;

            if bridge[x] {
                let obstacle_left = obstacle_prev(&obstacles[1 - y], x);
                let obstacle_right: usize = obstacle_next(&obstacles[1 - y], x, n);
                let cnt_non_bridge_total = tree.query_range(obstacle_left + 1, obstacle_right - 1);

                can_win_vertical = cnt_non_bridge_total == 0;
            }

            let mut can_win_endpoint = false;

            if obstacle_left + 1 != x && bridge[obstacle_left + 1] {
                let left_ok = obstacle_left + 1 > 1 && is_empty[1 - y][obstacle_left];
                let right_ok = obstacle_left + 1 < n && is_empty[1 - y][obstacle_left + 2];

                if left_ok || right_ok {
                    can_win_endpoint = true;
                }
            }

            if obstacle_right - 1 != x && bridge[obstacle_right - 1] {
                let left_ok = obstacle_right - 1 > 1 && is_empty[1 - y][obstacle_right - 2];
                let right_ok = obstacle_right - 1 < n && is_empty[1 - y][obstacle_right];

                if left_ok || right_ok {
                    can_win_endpoint = true;
                }
            }

            writeln!(
                out,
                "{}",
                if has_another_non_bridge || can_win_vertical || can_win_endpoint {
                    "Kaorin"
                } else {
                    "Turtle"
                }
            )
            .unwrap();
        }
    }
}
