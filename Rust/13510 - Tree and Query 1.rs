use io::Write;
use std::{cmp, io, str};

static mut COUNT: usize = 0;

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

#[derive(Clone)]
struct Node {
    id: usize,
    to: usize,
    cost: usize,
}

impl Node {
    fn new(id: usize, to: usize, cost: usize) -> Self {
        Self { id, to, cost }
    }
}

// Reference: https://www.secmem.org/blog/2019/12/12/HLD/
fn process_dfs_for_size(
    graph: &mut Vec<Vec<Node>>,
    size: &mut Vec<usize>,
    level: &mut Vec<usize>,
    cur_idx: usize,
    parent_idx: usize,
) {
    size[cur_idx] = 1;
    level[cur_idx] = level[parent_idx] + 1;

    for i in 0..graph[cur_idx].len() {
        if graph[cur_idx][i].to == parent_idx {
            continue;
        }

        process_dfs_for_size(graph, size, level, graph[cur_idx][i].to, cur_idx);
        size[cur_idx] += size[graph[cur_idx][i].to];

        if size[graph[cur_idx][i].to] > size[graph[cur_idx][0].to] {
            graph[cur_idx].swap(0, i);
        }
    }
}

unsafe fn process_dfs_for_hld(
    graph: &Vec<Vec<Node>>,
    tree: &mut Vec<usize>,
    num: &mut Vec<usize>,
    head: &mut Vec<usize>,
    p: &mut Vec<usize>,
    before: &mut Vec<usize>,
    n: usize,
    cur_idx: usize,
    parent_idx: usize,
) {
    num[cur_idx] = COUNT;

    for node in graph[cur_idx].iter() {
        if node.to == parent_idx {
            continue;
        }

        head[node.to] = if node.to == graph[cur_idx][0].to {
            head[cur_idx]
        } else {
            node.to
        };
        p[node.to] = cur_idx;
        COUNT += 1;

        process_update(tree, 1, 1, n, COUNT, node.cost);

        before[node.id] = COUNT;

        process_dfs_for_hld(graph, tree, num, head, p, before, n, node.to, cur_idx);
    }
}

fn process_update(
    tree: &mut Vec<usize>,
    node: usize,
    left: usize,
    right: usize,
    id: usize,
    val: usize,
) {
    if left > id || right < id {
        return;
    }

    if left == right {
        tree[node] = val;
        return;
    }

    let mid = (left + right) / 2;
    process_update(tree, node * 2, left, mid, id, val);
    process_update(tree, node * 2 + 1, mid + 1, right, id, val);

    tree[node] = cmp::max(tree[node * 2], tree[node * 2 + 1]);
}

fn find(
    tree: &Vec<usize>,
    node: usize,
    left: usize,
    right: usize,
    node_left: usize,
    node_right: usize,
) -> usize {
    if right < node_left || node_right < left || left > right {
        return 0;
    }

    if node_left <= left && right <= node_right {
        return tree[node];
    }

    let mid = (left + right) / 2;
    return cmp::max(
        find(tree, node * 2, left, mid, node_left, node_right),
        find(tree, node * 2 + 1, mid + 1, right, node_left, node_right),
    );
}

fn process_query(
    tree: &Vec<usize>,
    head: &Vec<usize>,
    level: &Vec<usize>,
    num: &Vec<usize>,
    p: &Vec<usize>,
    n: usize,
    mut a: usize,
    mut b: usize,
) -> usize {
    let mut ret = 0;

    while head[a] != head[b] {
        if level[head[a]] < level[head[b]] {
            std::mem::swap(&mut a, &mut b);
        }

        ret = cmp::max(ret, find(tree, 1, 1, n, num[head[a]], num[a]));
        a = p[head[a]];
    }

    if num[a] > num[b] {
        std::mem::swap(&mut a, &mut b);
    }

    cmp::max(ret, find(tree, 1, 1, n, num[a] + 1, num[b]))
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..n {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        graph[a].push(Node::new(i, b, c));
        graph[b].push(Node::new(i, a, c));
    }

    let mut size = vec![0; n + 1];
    let mut level = vec![0; n + 1];

    process_dfs_for_size(&mut graph, &mut size, &mut level, 1, 0);

    let mut tree = vec![0; (n + 1) * 4];
    let mut num = vec![0; n + 1];
    let mut head = vec![0; n + 1];
    let mut p = vec![0; n + 1];
    let mut before = vec![0; n + 1];

    unsafe {
        process_dfs_for_hld(
            &graph,
            &mut tree,
            &mut num,
            &mut head,
            &mut p,
            &mut before,
            n,
            1,
            0,
        );
    }

    let m = scan.token::<usize>();

    for _ in 0..m {
        let op = scan.token::<usize>();

        if op == 1 {
            let (i, c) = (scan.token::<usize>(), scan.token::<usize>());
            process_update(&mut tree, 1, 1, n, before[i], c);
        } else {
            let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
            writeln!(
                out,
                "{}",
                process_query(&tree, &head, &level, &num, &p, n, u, v)
            )
            .unwrap();
        }
    }
}
