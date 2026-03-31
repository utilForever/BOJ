use io::Write;
use std::{cmp::Ordering, collections::BinaryHeap, io, str};

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

#[derive(Clone, Eq, PartialEq)]
struct Item {
    depth: usize,
    cnt: i64,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth.cmp(&other.depth).then(self.cnt.cmp(&other.cnt))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Item {
    fn new(depth: usize, cnt: i64) -> Self {
        Self { depth, cnt }
    }
}

#[derive(Default)]
struct HeapState {
    heap: BinaryHeap<Item>,
    cnt_total: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut players = vec![0; n];

    for i in 0..n {
        players[i] = scan.token::<i64>();
    }

    let mut dungeons_root = Vec::new();
    let mut dungeons_child = vec![Vec::new(); n];

    for i in 0..n {
        let idx = scan.token::<usize>();

        if idx == 0 {
            dungeons_root.push(i);
        } else {
            dungeons_child[idx - 1].push(i);
        }
    }

    let mut is_boss_dungeon = vec![false; n];

    for _ in 0..m {
        let idx = scan.token::<usize>() - 1;
        is_boss_dungeon[idx] = true;
    }

    let mut depth = vec![0; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = Vec::new();

    for &root in dungeons_root.iter() {
        stack.push(root);
    }

    while let Some(node) = stack.pop() {
        order.push(node);

        for &next in dungeons_child[node].iter() {
            depth[next] = depth[node] + 1;
            stack.push(next);
        }
    }

    let mut states = (0..n).map(|_| None).collect::<Vec<_>>();

    for &u in order.iter().rev() {
        if is_boss_dungeon[u] {
            for &v in dungeons_child[u].iter() {
                states[v] = None;
            }

            let mut state = HeapState::default();

            if depth[u] < k {
                state.heap.push(Item::new(depth[u], players[u]));
                state.cnt_total = players[u];
            }

            states[u] = Some(state);
            continue;
        }

        let mut state_curr = HeapState::default();

        for &v in dungeons_child[u].iter() {
            let mut state_child = states[v].take().unwrap_or_default();

            if state_child.heap.len() > state_curr.heap.len() {
                std::mem::swap(&mut state_child, &mut state_curr);
            }

            state_curr.cnt_total += state_child.cnt_total;

            for item in state_child.heap.into_vec() {
                state_curr.heap.push(item);
            }
        }

        if state_curr.cnt_total > players[u] {
            let mut excess = state_curr.cnt_total - players[u];

            while excess > 0 {
                let mut top = state_curr.heap.pop().unwrap();

                if top.cnt <= excess {
                    excess -= top.cnt;
                    state_curr.cnt_total -= top.cnt;
                } else {
                    top.cnt -= excess;
                    state_curr.cnt_total -= excess;
                    excess = 0;
                    state_curr.heap.push(top);
                }
            }
        }

        states[u] = Some(state_curr);
    }

    let mut ret = 0;

    for root in dungeons_root {
        if let Some(state) = states[root].take() {
            for item in state.heap.into_vec() {
                ret += (k - item.depth) as i64 * item.cnt;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
