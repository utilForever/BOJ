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

static MOD: i64 = 998_244_353;

mod palindrome_tree {
    use std::collections::BTreeMap;

    #[derive(Clone, Default)]
    pub struct Node {
        pub len: i64,
        pub suff_link: usize,
        pub parent_link: usize,
        pub cnt: usize,
        pub dp: [i64; 3],
        next: BTreeMap<char, usize>,
    }

    impl Node {
        fn new(len: i64, suff_link: usize) -> Self {
            Self {
                len,
                suff_link,
                parent_link: 0,
                cnt: 0,
                dp: [0; 3],
                next: BTreeMap::new(),
            }
        }
    }

    pub struct PalindromicTree {
        pub nodes: Vec<Node>,
        str: String,
        pub cnt: usize,
        last_suff: usize,
    }

    impl PalindromicTree {
        pub fn new(str: &str, len: usize) -> Self {
            let mut tree = Self {
                nodes: vec![Node::default(); len],
                str: str.to_string(),
                cnt: 2,
                last_suff: 2,
            };

            tree.nodes[1] = Node::new(-1, 1);
            tree.nodes[2] = Node::new(0, 1);

            tree
        }

        pub fn init(&mut self) {
            let str = self.str.chars().collect::<Vec<_>>();

            for (idx, &c) in str.iter().enumerate() {
                let mut cur = self.last_suff;

                loop {
                    if idx as i64 - self.nodes[cur].len - 1 >= 0
                        && str[idx - self.nodes[cur].len as usize - 1] == c
                    {
                        break;
                    }

                    cur = self.nodes[cur].suff_link;
                }

                if self.nodes[cur].next.contains_key(&c) {
                    self.last_suff = self.nodes[cur].next[&c];
                    self.nodes[self.last_suff].cnt += 1;
                    continue;
                }

                self.cnt += 1;
                *self.nodes[cur].next.entry(c).or_insert(0) = self.cnt;
                self.last_suff = self.cnt;

                let next = self.cnt;
                self.nodes[next].parent_link = cur;
                self.nodes[next].len = self.nodes[cur].len + 2;
                self.nodes[next].cnt += 1;

                if self.nodes[next].len == 1 {
                    self.nodes[next].suff_link = 2;
                } else {
                    loop {
                        cur = self.nodes[cur].suff_link;

                        if idx as i64 - self.nodes[cur].len - 1 >= 0
                            && str[idx - self.nodes[cur].len as usize - 1] == c
                        {
                            self.nodes[next].suff_link = self.nodes[cur].next[&c];
                            break;
                        }
                    }
                }

                if self.nodes[next].cnt == 1 {
                    self.calculate_dp(next);
                }
            }
        }

        fn calculate_dp(&mut self, cur: usize) {
            use crate::MOD;

            let suff_link = self.nodes[cur].suff_link;
            let parent_link = self.nodes[cur].parent_link;

            self.nodes[cur].dp[1] = (self.nodes[parent_link].dp[2] + 1) % MOD;
            self.nodes[cur].dp[0] = (self.nodes[suff_link].dp[0] + self.nodes[cur].dp[1]) % MOD;

            let val1 = (self.nodes[parent_link].dp[2] + self.nodes[cur].dp[1]) % MOD;
            let val2 = (self.nodes[suff_link].dp[0] * 2) % MOD;
            self.nodes[cur].dp[2] = (val1 + val2) % MOD;
        }
    }
}

// Reference: https://www.secmem.org/blog/2019/05/17/Palindromic-Tree/
// Reference: CPC 2020 Editorial
fn main() {
    use palindrome_tree::PalindromicTree;

    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();

    let mut tree = PalindromicTree::new(&s, 1000010);
    tree.init();

    let mut ret = 0;

    for i in (3..=tree.cnt).rev() {
        let suff_link = tree.nodes[i].suff_link;
        tree.nodes[suff_link].cnt += tree.nodes[i].cnt;
        ret = ((tree.nodes[i].cnt as i64 * tree.nodes[i].dp[1]) % MOD + ret) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
