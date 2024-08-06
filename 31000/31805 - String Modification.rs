use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let mut s = s.chars().collect::<Vec<_>>();
    let mut idxes = vec![Vec::new(); 26];

    for i in 0..n {
        idxes[s[i] as usize - 'a' as usize].push(i);
    }

    let mut priority_queue = BinaryHeap::new();

    for i in 0..26 {
        if idxes[i].is_empty() {
            continue;
        }

        priority_queue.push((idxes[i].len(), i));
    }

    if priority_queue.peek().unwrap().0 > n / 2 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut ret = Vec::new();

    while priority_queue.len() > 1 {
        let (a_cnt, a_idx) = priority_queue.pop().unwrap();
        let (b_cnt, b_idx) = priority_queue.pop().unwrap();

        let idx1 = *idxes[a_idx].last().unwrap();
        let idx2 = *idxes[b_idx].last().unwrap();

        ret.push((idx1, idx2));
        s.swap(idx1, idx2);

        idxes[a_idx].pop();
        idxes[b_idx].pop();

        if a_cnt > 1 {
            priority_queue.push((a_cnt - 1, a_idx));
        }

        if b_cnt > 1 {
            priority_queue.push((b_cnt - 1, b_idx));
        }
    }

    if !priority_queue.is_empty() {
        let (_, idx_curr) = priority_queue.pop().unwrap();
        let idx_curr = *idxes[idx_curr].last().unwrap();
        let mut idx = 0;

        while idx < n && s[idx] == s[idx_curr] {
            idx += 1;
        }

        ret.push((idx, idx_curr));
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for (a, b) in ret {
        writeln!(out, "{} {}", (a + 1).min(b + 1), (a + 1).max(b + 1)).unwrap();
    }
}
