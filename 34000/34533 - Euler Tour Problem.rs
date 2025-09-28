use io::Write;
use std::{cmp::Ordering, io, str};

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

fn process_suffix_array(s: &Vec<char>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = s.len();
    let mut suffix_array: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i64> = s.iter().map(|&c| c as i64).collect();
    let mut rank_new = vec![0; n];
    let mut k = 1;

    while k < n {
        suffix_array.sort_unstable_by(|&a, &b| {
            let ka = (rank[a], if a + k < n { rank[a + k] } else { -1 });
            let kb = (rank[b], if b + k < n { rank[b + k] } else { -1 });

            ka.cmp(&kb)
        });

        rank_new[suffix_array[0]] = 0;

        for i in 1..n {
            let prev = suffix_array[i - 1];
            let curr = suffix_array[i];

            let key_prev = (rank[prev], if prev + k < n { rank[prev + k] } else { -1 });
            let key_curr = (rank[curr], if curr + k < n { rank[curr + k] } else { -1 });

            rank_new[curr] = rank_new[prev] + if key_prev != key_curr { 1 } else { 0 };
        }

        rank.clone_from_slice(&rank_new);

        if rank[suffix_array[n - 1]] as usize == n - 1 {
            break;
        }

        k <<= 1;
    }

    let mut lcp_array = vec![0; n];
    let mut h = 0;

    for i in 0..n {
        let r = rank[i] as usize;

        if r == 0 {
            continue;
        }

        let j = suffix_array[r - 1];

        while i + h < n && j + h < n && s[i + h] == s[j + h] {
            h += 1;
        }

        lcp_array[r] = h;

        if h > 0 {
            h -= 1;
        }
    }

    (
        suffix_array,
        rank.iter().map(|&x| x as usize).collect(),
        lcp_array,
    )
}

struct RMQ {
    sparse_table: Vec<Vec<usize>>,
    log_table: Vec<usize>,
}

impl RMQ {
    fn new(arr: &Vec<usize>) -> Self {
        let n = arr.len();
        let mut log_table = vec![0; n + 1];

        for i in 2..=n {
            log_table[i] = log_table[i / 2] + 1;
        }

        let mut sparse_table = vec![vec![0; n]; log_table[n] + 1];
        sparse_table[0].clone_from_slice(arr);

        for i in 1..=log_table[n] {
            let len = 1usize << i;

            for j in 0..=n.saturating_sub(len) {
                sparse_table[i][j] =
                    sparse_table[i - 1][j].min(sparse_table[i - 1][j + (len >> 1)]);
            }
        }

        Self {
            sparse_table,
            log_table,
        }
    }

    fn query(&self, left: usize, right: usize) -> usize {
        if left > right {
            return usize::MAX;
        }

        let len = right - left + 1;
        let k = self.log_table[len];

        self.sparse_table[k][left].min(self.sparse_table[k][right + 1 - (1usize << k)])
    }
}

#[inline]
fn lcp_between_suffixes(rank: &Vec<usize>, rmq: &RMQ, i: usize, j: usize, len: usize) -> usize {
    if i == j {
        return len - i;
    }

    let mut rank_i = rank[i];
    let mut rank_j = rank[j];

    if rank_i > rank_j {
        std::mem::swap(&mut rank_i, &mut rank_j);
    }

    rmq.query(rank_i + 1, rank_j)
}

#[derive(Clone)]
struct Candidate {
    node: usize,
    idx_first_diff: usize,
    suffix_from_diff: Vec<(usize, usize)>,
}

impl Candidate {
    fn new(node: usize, idx_first_diff: usize, suffix_from_diff: Vec<(usize, usize)>) -> Self {
        Self {
            node,
            idx_first_diff,
            suffix_from_diff,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut parent = vec![-1; n + 1];

        for i in 1..=n {
            parent[i] = scan.token::<i64>();
        }

        let mut children = vec![Vec::new(); n + 1];

        for i in 2..=n {
            let p = parent[i] as usize;
            children[p].push(i);
        }

        for i in 1..=n {
            children[i].sort_unstable();
        }

        let mut trace = Vec::with_capacity(n * 2);
        let mut pos_enter = vec![0; n + 1];
        let mut pos_exit = vec![0; n + 1];

        let mut stack = Vec::new();
        let mut idx_next = vec![0; n + 1];

        stack.push(1);

        while let Some(&node) = stack.last() {
            let idx = idx_next[node];

            if idx == 0 {
                pos_enter[node] = trace.len();
                trace.push('1');
            }

            if idx < children[node].len() {
                let next = children[node][idx];

                idx_next[node] += 1;
                stack.push(next);
                idx_next[next] = 0;
            } else {
                pos_exit[node] = trace.len();
                trace.push('0');
                stack.pop();
            }
        }

        let mut nodes_subtree = vec![0; n + 1];
        let mut stack = Vec::new();

        stack.push((1, false));

        while let Some((node, visited)) = stack.pop() {
            if visited {
                let mut sum = 1;

                for &next in children[node].iter() {
                    sum += nodes_subtree[next];
                }

                nodes_subtree[node] = sum;
            } else {
                stack.push((node, true));

                for &next in children[node].iter().rev() {
                    stack.push((next, false));
                }
            }
        }

        let len = trace.len();
        let (_, suffix_rank, lcp_array) = process_suffix_array(&trace);
        let rmq = RMQ::new(&lcp_array);

        let lcp_bounded = |pos1: usize, rem1: usize, pos2: usize, rem2: usize| -> usize {
            let lcp = lcp_between_suffixes(&suffix_rank, &rmq, pos1, pos2, len);
            lcp.min(rem1).min(rem2)
        };

        let compare_segmented_strings =
            |a: &Vec<(usize, usize)>, b: &Vec<(usize, usize)>| -> Ordering {
                let (mut idx_a, mut idx_b) = (0, 0);
                let (mut offset_a, mut offset_b) = (0, 0);

                while idx_a < a.len() && idx_b < b.len() {
                    let (pos_a, nodes_a) = a[idx_a];
                    let (pos_b, nodes_b) = b[idx_b];
                    let rem_a = nodes_a - offset_a;
                    let rem_b = nodes_b - offset_b;
                    let lcp = lcp_bounded(pos_a + offset_a, rem_a, pos_b + offset_b, rem_b);

                    if lcp < rem_a && lcp < rem_b {
                        let char_a = trace[pos_a + offset_a + lcp];
                        let char_b = trace[pos_b + offset_b + lcp];

                        return char_a.cmp(&char_b);
                    }

                    offset_a += lcp;
                    offset_b += lcp;

                    if offset_a == nodes_a {
                        idx_a += 1;
                        offset_a = 0;
                    }

                    if offset_b == nodes_b {
                        idx_b += 1;
                        offset_b = 0;
                    }
                }

                if idx_a == a.len() && idx_b == b.len() {
                    Ordering::Equal
                } else if idx_a == a.len() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            };

        let compare_concate_order = |a: usize, b: usize| -> Ordering {
            let segment1 = [
                (pos_enter[a], 2 * nodes_subtree[a]),
                (pos_enter[b], 2 * nodes_subtree[b]),
            ];
            let segment2 = [
                (pos_enter[b], 2 * nodes_subtree[b]),
                (pos_enter[a], 2 * nodes_subtree[a]),
            ];

            let (mut idx1, mut idx2) = (0, 0);
            let (mut offset1, mut offset2) = (0, 0);

            loop {
                if idx1 == 2 && idx2 == 2 {
                    return Ordering::Equal;
                }

                let (pos1, nodes1) = segment1[idx1];
                let (pos2, nodes2) = segment2[idx2];
                let rem1 = nodes1 - offset1;
                let rem2 = nodes2 - offset2;
                let lcp = lcp_bounded(pos1 + offset1, rem1, pos2 + offset2, rem2);

                if lcp < rem1 && lcp < rem2 {
                    let char1 = trace[pos1 + offset1 + lcp];
                    let char2 = trace[pos2 + offset2 + lcp];

                    return char1.cmp(&char2);
                }

                offset1 += lcp;
                offset2 += lcp;

                if offset1 == nodes1 {
                    idx1 += 1;
                    offset1 = 0;
                }

                if offset2 == nodes2 {
                    idx2 += 1;
                    offset2 = 0;
                }
            }
        };

        let mut best_children = vec![Vec::new(); n + 1];

        for i in 1..=n {
            if children[i].len() <= 1 {
                best_children[i] = children[i].clone();
                continue;
            }

            let mut list = children[i].clone();

            list.sort_unstable_by(|&a, &b| {
                let ord = compare_concate_order(a, b);

                if ord == Ordering::Equal {
                    suffix_rank[pos_enter[a]].cmp(&suffix_rank[pos_enter[b]])
                } else {
                    ord
                }
            });

            best_children[i] = list;
        }

        let mut best: Option<Candidate> = None;

        for node in 1..=n {
            let original = &children[node];
            let minimal_order = &best_children[node];

            if original.len() <= 1 || original == minimal_order {
                continue;
            }

            let (mut i, mut j) = (0, 0);
            let (mut offset_i, mut offset_j) = (0, 0);
            let mut consumed = 0;
            let mut candidate = None;

            while i < original.len() && j < minimal_order.len() {
                let a = original[i];
                let b = minimal_order[j];
                let pos_a = pos_enter[a] + offset_i;
                let pos_b = pos_enter[b] + offset_j;
                let rem_a = 2 * nodes_subtree[a] - offset_i;
                let rem_b = 2 * nodes_subtree[b] - offset_j;
                let lcp = lcp_bounded(pos_a, rem_a, pos_b, rem_b);

                if lcp < rem_a && lcp < rem_b {
                    let c_old = trace[pos_a + lcp];
                    let c_new = trace[pos_b + lcp];

                    if c_old > c_new {
                        let idx_first_diff = pos_enter[node] + 1 + consumed + lcp;
                        let mut segments = Vec::new();

                        if rem_b - lcp > 0 {
                            segments.push((pos_b + lcp, rem_b - lcp));
                        }

                        for k in j + 1..minimal_order.len() {
                            let c = minimal_order[k];
                            segments.push((pos_enter[c], 2 * nodes_subtree[c]));
                        }

                        segments.push((pos_exit[node], len - pos_exit[node]));
                        candidate = Some(Candidate::new(node, idx_first_diff, segments));
                    }

                    break;
                }

                consumed += lcp;
                offset_i += lcp;
                offset_j += lcp;

                if offset_i == 2 * nodes_subtree[a] {
                    i += 1;
                    offset_i = 0;
                }

                if offset_j == 2 * nodes_subtree[b] {
                    j += 1;
                    offset_j = 0;
                }
            }

            if let Some(c) = candidate {
                match &mut best {
                    Some(curr) => {
                        if c.idx_first_diff < curr.idx_first_diff {
                            *curr = c;
                        } else if c.idx_first_diff == curr.idx_first_diff {
                            let ord = compare_segmented_strings(
                                &c.suffix_from_diff,
                                &curr.suffix_from_diff,
                            );

                            if ord == Ordering::Less {
                                *curr = c;
                            }
                        }
                    }
                    None => best = Some(c),
                }
            }
        }

        let ret = if let Some(best) = best {
            let mut stack = Vec::new();
            let mut val = Vec::with_capacity(len);

            stack.push((1, 0));

            while let Some((node, idx)) = stack.pop() {
                if idx == 0 {
                    val.push('1');
                }

                let list = if node == best.node {
                    &best_children[node]
                } else {
                    &children[node]
                };

                if idx < list.len() {
                    stack.push((node, idx + 1));
                    stack.push((list[idx], 0));
                } else {
                    val.push('0');
                }
            }

            val.iter().collect::<String>()
        } else {
            trace.iter().collect::<String>()
        };

        writeln!(out, "{ret}").unwrap();
    }
}
