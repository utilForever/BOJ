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

#[derive(Clone, Default)]
struct Node {
    len: i64,
    suff_link: usize,
    series_link: usize,
    diff: i64,
    next: [usize; 26],
}

impl Node {
    fn new(len: i64, suff_link: usize, series_link: usize, diff: i64) -> Self {
        Self {
            len,
            suff_link,
            series_link,
            diff,
            next: [0; 26],
        }
    }
}

pub struct PalindromeTree {
    nodes: Vec<Node>,
    last_suff: usize,
}

impl PalindromeTree {
    pub fn new() -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            last_suff: 2,
        };

        tree.nodes.push(Node::new(0, 0, 0, 0));
        tree.nodes.push(Node::new(-1, 1, 1, -1));
        tree.nodes.push(Node::new(0, 1, 1, 0));

        tree
    }

    pub fn palindromic_length(&mut self, s: &str) -> i64 {
        let n = s.len();
        let chars = s.chars().collect::<Vec<_>>();
        let mut add = vec![i64::MAX; n + 3];
        let mut dp = vec![i64::MAX; n + 1];

        dp[0] = 0;

        for i in 0..n {
            let idx = chars[i] as usize - 'a' as usize;
            let mut cur = self.last_suff;

            loop {
                let len_cur = self.nodes[cur].len as usize;

                if i >= len_cur + 1 && chars[i - len_cur - 1] == chars[i] {
                    break;
                }

                cur = self.nodes[cur].suff_link;
            }

            if self.nodes[cur].next[idx] != 0 {
                self.last_suff = self.nodes[cur].next[idx];
            } else {
                let v = self.nodes.len();

                self.nodes.push(Node::new(self.nodes[cur].len + 2, 0, 0, 0));
                self.nodes[cur].next[idx] = v;

                if self.nodes[v].len == 1 {
                    self.nodes[v].suff_link = 2;
                } else {
                    let mut link_cur = self.nodes[cur].suff_link;

                    loop {
                        let link_len = self.nodes[link_cur].len as usize;

                        if i >= link_len + 1 && chars[i - link_len - 1] == chars[i] {
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

                self.last_suff = v;
            }

            dp[i + 1] = n as i64 + 1;

            let mut v = self.last_suff;

            while v > 2 {
                let len_series = self.nodes[self.nodes[v].series_link].len as usize;
                let prev = i + 1 - (len_series + self.nodes[v].diff as usize);

                add[v] = dp[prev] + 1;

                if self.nodes[v].diff == self.nodes[self.nodes[v].suff_link].diff {
                    add[v] = add[v].min(add[self.nodes[v].suff_link]);
                }

                dp[i + 1] = dp[i + 1].min(add[v]);
                v = self.nodes[v].series_link;
            }
        }

        dp[n]
    }
}

// Reference: https://arxiv.org/pdf/1506.04862
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let _ = scan.token::<usize>();
    let s = scan.token::<String>();
    let mut tree = PalindromeTree::new();
    let ret = tree.palindromic_length(&s);

    writeln!(out, "{ret}").unwrap();
}
