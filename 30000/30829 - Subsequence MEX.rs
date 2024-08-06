use io::Write;
use std::{collections::VecDeque, io, str};

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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut num_next = vec![vec![0; 10]; s.len() + 1];

    for i in 0..=9 {
        num_next[s.len()][i] = -1;
    }

    for i in (0..s.len()).rev() {
        for j in 0..=9 {
            num_next[i][j] = num_next[i + 1][j];
        }

        num_next[i][s[i] as usize - '0' as usize] = i as i64 + 1;
    }

    let mut queue = VecDeque::new();
    let mut check = vec![false; s.len() + 1];
    let mut backtrack = vec![(0, 0); s.len() + 1];

    queue.push_back(0);

    while !queue.is_empty() {
        let val = queue.pop_front().unwrap();
        let mut idx = if val == 0 { 1 } else { 0 };

        while idx <= 9 {
            if num_next[val][idx] == -1 {
                let mut ret = String::new();
                ret.push((idx as u8 + '0' as u8) as char);

                let mut curr = val;

                loop {
                    if curr == 0 {
                        writeln!(out, "{}", ret.chars().rev().collect::<String>()).unwrap();
                        return;
                    }

                    ret.push((backtrack[curr].1 as u8 + '0' as u8) as char);
                    curr = backtrack[curr].0;
                }
            }

            if check[num_next[val][idx] as usize] {
                idx += 1;
                continue;
            }

            queue.push_back(num_next[val][idx] as usize);
            check[num_next[val][idx] as usize] = true;
            backtrack[num_next[val][idx] as usize] = (val, idx);

            idx += 1;
        }
    }
}
