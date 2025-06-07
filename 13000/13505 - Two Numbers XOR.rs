#![allow(dead_code)]

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

const BIT_LENGTH: usize = 31;

struct Node {
    children: [Option<usize>; 2],
    count: i32,
}

struct BinaryTrie {
    nodes: Vec<Node>,
}

impl BinaryTrie {
    fn new() -> Self {
        let root = Node {
            children: [None, None],
            count: 0,
        };

        Self { nodes: vec![root] }
    }

    fn insert(&mut self, x: i32) {
        let mut cur = 0;
        self.nodes[cur].count += 1;

        for i in (0..BIT_LENGTH).rev() {
            let bit = ((x >> i) & 1) as usize;

            if self.nodes[cur].children[bit].is_none() {
                self.nodes.push(Node {
                    children: [None, None],
                    count: 0,
                });

                let new_index = self.nodes.len() - 1;
                self.nodes[cur].children[bit] = Some(new_index);
            }

            cur = self.nodes[cur].children[bit].unwrap();
            self.nodes[cur].count += 1;
        }
    }

    fn remove(&mut self, x: i32) {
        let mut cur = 0;
        self.nodes[cur].count -= 1;

        for i in (0..BIT_LENGTH).rev() {
            let bit = ((x >> i) & 1) as usize;
            cur = self.nodes[cur].children[bit].unwrap();
            self.nodes[cur].count -= 1;
        }
    }

    fn query(&self, x: i32) -> i32 {
        let mut cur = 0;
        let mut ret = 0;

        for i in (0..BIT_LENGTH).rev() {
            let bit = ((x >> i) & 1) as usize;
            let toggled = 1 - bit;

            if let Some(next) = self.nodes[cur].children[toggled] {
                if self.nodes[next].count > 0 {
                    ret |= 1 << i;
                    cur = next;
                    continue;
                }
            }

            cur = self.nodes[cur].children[bit].unwrap();
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut trie = BinaryTrie::new();
    let mut ret = 0;

    for _ in 0..n {
        let val = scan.token::<i32>();
        trie.insert(val);

        ret = ret.max(trie.query(val));
    }

    writeln!(out, "{ret}").unwrap();
}
