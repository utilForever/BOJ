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

fn process_suffix_array(s: &Vec<char>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = s.len();
    let mut suffix_array: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = s.iter().map(|&c| (c as usize) + 1).collect();
    let mut tmp = vec![0; n];
    let mut k = 1;

    while k < n {
        suffix_array.sort_by(|&a, &b| {
            if rank[a] != rank[b] {
                return rank[a].cmp(&rank[b]);
            }

            let ra = if a + k < n { rank[a + k] } else { 0 };
            let rb = if b + k < n { rank[b + k] } else { 0 };

            if ra != rb {
                ra.cmp(&rb)
            } else {
                b.cmp(&a)
            }
        });

        tmp[suffix_array[0]] = 0;

        for i in 1..n {
            let prev = suffix_array[i - 1];
            let curr = suffix_array[i];

            let key_prev = (rank[prev], if prev + k < n { rank[prev + k] } else { 0 });
            let key_curr = (rank[curr], if curr + k < n { rank[curr + k] } else { 0 });

            tmp[curr] = tmp[prev] + if key_prev != key_curr { 1 } else { 0 };
        }

        rank.clone_from_slice(&tmp);

        if rank[suffix_array[n - 1]] == n - 1 {
            break;
        }

        k <<= 1;
    }

    let mut lcp_array = vec![0; n];
    let mut h = 0;

    for i in 0..n {
        let r = rank[i];

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

    (suffix_array, rank, lcp_array)
}

fn process_manacher(text: &Vec<char>) -> (Vec<Vec<usize>>, usize) {
    let n = text.len();
    let m = 2 * n + 1;
    let mut t = vec!['*'; m];

    for i in 0..n {
        t[i * 2 + 1] = text[i];
    }

    let mut p = vec![0; m];
    let mut c = 0;
    let mut r = 0;

    let mut bucket: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut best = 1;

    for i in 0..m {
        if i < r {
            p[i] = p[2 * c - i].min(r - i);
        }

        while i + p[i] + 1 < m && i >= p[i] + 1 && t[i + p[i] + 1] == t[i - p[i] - 1] {
            p[i] += 1;
        }

        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }

        best = best.max(p[i]);

        if p[i] == 0 {
            continue;
        }

        let right = (i + p[i]) / 2;

        if right >= n {
            continue;
        }

        let left = ((i - p[i]) / 2).wrapping_sub(1);

        if left < n {
            bucket[right].push(left);
        }
    }

    (bucket, best)
}

struct SegMin {
    size: usize,
    data: Vec<usize>,
}

impl SegMin {
    fn new(n: usize) -> Self {
        let mut size = 1;

        while size < n {
            size <<= 1;
        }

        Self {
            size,
            data: vec![usize::MAX; size << 1],
        }
    }

    fn build(&mut self) {
        for i in (1..self.size).rev() {
            self.data[i] = self.data[i << 1].min(self.data[i << 1 | 1]);
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> usize {
        if left > right {
            return 0;
        }

        left += self.size;
        right += self.size;

        let mut ret = usize::MAX;

        while left <= right {
            if left & 1 == 1 {
                ret = ret.min(self.data[left]);
                left += 1;
            }

            if right & 1 == 0 {
                ret = ret.min(self.data[right]);

                if right == 0 {
                    break;
                }

                right -= 1;
            }

            left >>= 1;
            right >>= 1;
        }

        ret
    }
}

fn process(s: &Vec<char>) -> usize {
    let n = s.len();

    if n == 1 {
        return 1;
    }

    let (bucket, best) = process_manacher(s);

    let mut t = Vec::with_capacity(2 * n + 1);
    t.extend_from_slice(s);
    t.push('*');
    t.extend(s.iter().rev());

    let (_, rank, lcp) = process_suffix_array(&t);
    let mut seg = SegMin::new(2 * n);

    for i in 1..(2 * n + 1) {
        seg.data[i - 1 + seg.size] = lcp[i];
    }

    seg.build();

    let mut ret = best;
    let mut set = BTreeSet::new();

    for i in (0..n).rev() {
        set.insert(rank[i]);

        for &x in bucket[i].iter() {
            let y = 2 * n - x;
            let u = rank[y];

            if let Some(&v) = set.range((u + 1)..).next() {
                let pos = seg.query(u.min(v), u.max(v) - 1);
                ret = ret.max(i - x - 1 + 2 * pos);
            }

            if let Some(&v) = set.range(..=u).next_back() {
                let pos = seg.query(u.min(v), u.max(v) - 1);
                ret = ret.max(i - x - 1 + 2 * pos);
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let ret1 = process(&s);
    let ret2 = process(&s.iter().rev().cloned().collect::<Vec<_>>());

    writeln!(out, "{}", ret1.max(ret2)).unwrap();
}
