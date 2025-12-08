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

#[derive(Debug, Default)]
struct TrieNode {
    is_end_of_word: bool,
    children: Vec<(u8, usize)>,
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut idx = 0;

        for c in word.bytes() {
            let index = c - b'a';
            let mut next = None;
            let children = &self.nodes[idx].children;

            for &(c, idx_child) in children.iter() {
                if c == index {
                    next = Some(idx_child);
                    break;
                }
            }

            let idx_next = match next {
                Some(idx) => idx,
                None => {
                    let idx_new = self.nodes.len();

                    self.nodes.push(TrieNode::default());
                    self.nodes[idx].children.push((index, idx_new));

                    idx_new
                }
            };

            idx = idx_next;
        }

        self.nodes[idx].is_end_of_word = true;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut trie = Trie::new();

    for _ in 0..n {
        let mut word = scan.token::<String>();
        word = word.chars().rev().collect();

        trie.insert(&word);
    }

    let mut order = Vec::with_capacity(trie.nodes.len());
    let mut stack = Vec::new();

    stack.push(0);

    while let Some(u) = stack.pop() {
        order.push(u);

        for &(_, v) in trie.nodes[u].children.iter() {
            stack.push(v);
        }
    }

    let mut dp = vec![0; trie.nodes.len()];
    let mut ret = 0;

    for &u in order.iter().rev() {
        let node = &trie.nodes[u];
        let mut sum = 0;
        let mut max_first = 0;
        let mut max_second = 0;

        for &(_, v) in node.children.iter() {
            if trie.nodes[v].is_end_of_word {
                sum += 1;
            }

            if dp[v] >= max_first {
                max_second = max_first;
                max_first = dp[v];
            } else if dp[v] > max_second {
                max_second = dp[v];
            }
        }

        dp[u] = if node.is_end_of_word {
            max_first + if sum >= 1 { sum } else { 1 }
        } else {
            0
        };

        let extra = if sum >= 2 { sum - 2 } else { 0 };
        let cand = (if node.is_end_of_word { 1 } else { 0 }) + max_first + max_second + extra;
        ret = ret.max(cand);
    }

    writeln!(out, "{ret}").unwrap();
}
