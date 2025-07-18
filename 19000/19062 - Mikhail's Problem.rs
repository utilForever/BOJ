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
}

#[derive(Clone, Default)]
struct Node {
    len: i64,           // length of this palindrome
    suff_link: usize,   // longest proper palindromic suffix
    series_link: usize, // next node with diff != current diff
    leading: usize,     // leading of the diff‑block
    diff: i64,          // len – len[suff_link]
    next: [usize; 26],  // transitions by adding one char to both ends
}

impl Node {
    fn new(len: i64, suff_link: usize, series_link: usize, diff: i64) -> Self {
        Self {
            len,
            suff_link,
            series_link,
            leading: 0,
            diff,
            next: [0; 26],
        }
    }
}

pub struct PalindromeTree {
    nodes: Vec<Node>,
    chars: Vec<char>,
    last_suff: usize,               // node for current max suffix‑palindrome
    fenwick: FenwickTree,           // tree over left‑end positions
    stacks: Vec<Vec<(usize, i64)>>, // leading-idxes → stack of (left, len)
}

impl PalindromeTree {
    pub fn new(s: &String) -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            last_suff: 1,
            chars: s.chars().collect(),
            fenwick: FenwickTree::new(s.len()),
            stacks: Vec::new(),
        };

        tree.nodes.push(Node::new(-1, 0, 0, 0));
        tree.nodes.push(Node::new(0, 0, 0, 0));
        tree.stacks.push(Vec::new());
        tree.stacks.push(Vec::new());

        tree
    }

    pub fn add_char(&mut self, pos: usize) -> (Vec<usize>, Vec<usize>) {
        let idx = self.chars[pos] as usize - 'a' as usize;
        let mut cur = self.last_suff;

        loop {
            let len_cur = self.nodes[cur].len as usize;

            if pos >= len_cur + 1 && self.chars[pos - len_cur - 1] == self.chars[pos] {
                break;
            }

            cur = self.nodes[cur].suff_link;
        }

        if self.nodes[cur].next[idx] == 0 {
            let v = self.nodes.len();

            self.nodes.push(Node::new(self.nodes[cur].len + 2, 0, 0, 0));
            self.stacks.push(Vec::new());
            self.nodes[cur].next[idx] = v;

            if self.nodes[v].len == 1 {
                self.nodes[v].suff_link = 1;
            } else {
                let mut link_cur = self.nodes[cur].suff_link;

                loop {
                    let link_len = self.nodes[link_cur].len as usize;

                    if pos >= link_len + 1 && self.chars[pos - link_len - 1] == self.chars[pos] {
                        self.nodes[v].suff_link = self.nodes[link_cur].next[idx];
                        break;
                    }

                    link_cur = self.nodes[link_cur].suff_link;
                }
            }

            let suff_link = self.nodes[v].suff_link;

            self.nodes[v].diff = self.nodes[v].len - self.nodes[suff_link].len;
            self.nodes[v].series_link = if self.nodes[v].diff == self.nodes[suff_link].diff {
                self.nodes[suff_link].series_link
            } else {
                suff_link
            };
            self.nodes[v].leading = if self.nodes[v].diff == self.nodes[suff_link].diff {
                self.nodes[suff_link].leading
            } else {
                v
            };
        }

        self.last_suff = self.nodes[cur].next[idx];

        // Traverse the series‑link chain
        // For each leading, decide which left positions switch on/off
        // add: left positions whose leading just became active (+1)
        // del: positions that just became obsolete (−1)
        let mut add = Vec::new();
        let mut del = Vec::new();
        let mut v = self.last_suff;

        while v > 1 {
            let leading = self.nodes[v].leading;
            let stack = &mut self.stacks[leading];
            let left_add = pos + 2 - self.nodes[v].len as usize;

            // Push (left_add, len[v]) onto stack (merging if same left twice)
            if let Some(last) = stack.last_mut() {
                if last.0 == left_add {
                    last.1 = self.nodes[v].len;
                } else {
                    stack.push((left_add, self.nodes[v].len));
                }
            } else {
                stack.push((left_add, self.nodes[v].len));
            }

            // Leading's own left becomes active now
            add.push(pos + 2 - self.nodes[leading].len as usize);

            // If stack has >= 2 elements, the previous interval ends -> deactivate
            if stack.len() >= 2 {
                let (left_prev, len_prev) = stack[stack.len() - 2];
                let right_prev = left_prev + len_prev as usize - 1;
                let left_del = right_prev + 1 - self.nodes[v].len as usize;

                del.push(left_del);
            }

            // If two consecutive intervals have the same length, keep only the newer one
            if stack.len() >= 2 && stack[stack.len() - 1].1 == stack[stack.len() - 2].1 {
                let (left, _) = stack.pop().unwrap();
                stack.last_mut().unwrap().0 = left;
            }

            v = self.nodes[v].series_link;
        }

        (add, del)
    }

    fn process_queries(&mut self, queries: &[Vec<Query>], q: usize) -> Vec<i64> {
        let mut bit_flag = vec![0; self.chars.len() + 2];
        let mut pos = 0;
        let mut ret = vec![0; q];

        for (r, query) in queries.iter().enumerate() {
            while pos < r {
                let (add, del) = self.add_char(pos);

                // Deactivate obsolete leadings
                for &idx in del.iter() {
                    if bit_flag[idx] == 1 {
                        bit_flag[idx] = 0;
                        self.fenwick.update(idx, -1);
                    }
                }

                // Activate new leadings
                for &idx in add.iter() {
                    if bit_flag[idx] == 0 {
                        bit_flag[idx] = 1;
                        self.fenwick.update(idx, 1);
                    }
                }

                pos += 1;
            }

            if query.is_empty() {
                continue;
            }

            let total = (self.nodes.len() - 2) as i64;

            for &Query { l, idx } in query {
                ret[idx] = total - self.fenwick.query(l - 1);
            }
        }

        ret
    }
}

#[derive(Clone)]
struct Query {
    l: usize,
    idx: usize,
}

impl Query {
    fn new(l: usize, idx: usize) -> Self {
        Self { l, idx }
    }
}

// Reference: https://arxiv.org/pdf/1506.04862
// Reference: https://www.cnblogs.com/clrs97/p/8667455.html
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (s, q) = (scan.token::<String>(), scan.token::<usize>());
    let mut queries = vec![Vec::new(); s.len() + 1];

    for i in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        queries[r].push(Query::new(l, i));
    }

    let mut tree = PalindromeTree::new(&s);
    let ret = tree.process_queries(&queries, q);

    for i in 0..q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}
