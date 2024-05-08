use io::Write;
use std::{collections::HashMap, io, str};

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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut start = vec![(0, 0); n];
    let mut end = vec![(0, 0); n];

    for i in 0..n {
        let (l, r, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );
        start[i] = (l, c);
        end[i] = (r, c);
    }

    start.sort();
    end.sort();

    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut idx = 0;
    let mut idx2 = 0;
    let mut cnt = 0;
    let mut ret = 0;

    while idx < n {
        let mut idx1 = idx;

        while idx1 < n && start[idx].0 == start[idx1].0 {
            map.entry(start[idx1].1)
                .and_modify(|x| *x += 1)
                .or_insert(1);

            if map.contains_key(&start[idx1].1) && map[&start[idx1].1] == 2 {
                cnt += 1;
            }

            idx1 += 1;
        }

        while idx2 < n && end[idx2].0 <= start[idx].0 - m {
            map.entry(end[idx2].1).and_modify(|x| *x -= 1);

            if map.contains_key(&end[idx2].1) && map[&end[idx2].1] == 1 {
                cnt -= 1;
            }

            idx2 += 1;
        }

        ret = ret.max(cnt);
        idx = idx1;
    }

    writeln!(out, "{ret}").unwrap();
}
