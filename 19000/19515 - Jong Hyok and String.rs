use io::Write;
use std::{collections::BTreeMap, io, str};

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
pub struct SuffixAutomaton {
    pub st: Vec<SAState>,
    pub sz: usize,
    pub last: usize,
}

#[derive(Default, Clone)]
pub struct SAState {
    pub len: usize,
    pub link: i32,
    pub next: BTreeMap<char, usize>,
}

impl SuffixAutomaton {
    pub fn from_str(s: &str) -> Self {
        let mut sa = Self::new(s.len());

        for ch in s.chars() {
            sa.add(ch);
        }

        sa
    }

    pub fn new(n: usize) -> Self {
        let mut sa = Self {
            st: vec![],
            sz: 1,
            last: 0,
        };

        for _ in 0..(2 * n) {
            sa.st.push(SAState::default());
        }

        sa.st[0].len = 0;
        sa.st[0].link = -1;

        sa
    }

    pub fn add(&mut self, c: char) {
        let cur = self.sz;

        self.sz += 1;
        self.st[cur].len = self.st[self.last].len + 1;

        let mut p = self.last as i32;

        while p != -1 && !self.st[p as usize].next.contains_key(&c) {
            self.st[p as usize].next.insert(c, cur);
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
        } else {
            let pu = p as usize;
            let q = self.st[pu].next[&c];

            if self.st[pu].len + 1 == self.st[q].len {
                self.st[cur].link = q as i32;
            } else {
                let clone = self.sz;

                self.sz += 1;
                self.st[clone].len = self.st[pu].len + 1;
                self.st[clone].next = self.st[q].next.clone();
                self.st[clone].link = self.st[q].link;

                while p != -1 && *self.st[p as usize].next.get(&c).unwrap() == q {
                    self.st[p as usize].next.insert(c, clone);
                    p = self.st[p as usize].link;
                }

                self.st[cur].link = clone as i32;
                self.st[q].link = self.st[cur].link;
            }
        }

        self.last = cur;
    }

    pub fn count_unique_substring(&self) -> i64 {
        let mut tot = 0;

        for i in 1..self.sz {
            tot += (self.st[i].len - self.st[self.st[i].link as usize].len) as i64;
        }

        tot
    }
}

// Reference: https://github.com/heiseish/Competitive-Programming
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut strings = Vec::with_capacity(n);

    for _ in 0..n {
        strings.push(scan.token::<String>());
    }

    let total = strings.iter().map(|s| s.len()).sum::<usize>();
    let mut sa = SuffixAutomaton::new(total);

    for string in strings {
        sa.last = 0;

        for c in string.chars() {
            sa.add(c);
        }
    }

    for _ in 0..m {
        let s = scan.token::<String>();
        let mut pos = 0;
        let mut exist = true;

        for c in s.chars() {
            if let Some(&next) = sa.st[pos].next.get(&c) {
                pos = next;
            } else {
                exist = false;
                break;
            }
        }

        if !exist {
            writeln!(out, "0").unwrap();
            continue;
        }

        let link = sa.st[pos].link;
        let link_len = if link == -1 {
            0
        } else {
            sa.st[link as usize].len
        };
        let ret = sa.st[pos].len as i64 - link_len as i64;

        writeln!(out, "{ret}").unwrap();
    }
}
