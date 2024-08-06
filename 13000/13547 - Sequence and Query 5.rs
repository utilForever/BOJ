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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token();
    let mut arr: Vec<usize> = vec![0; 100_001];

    for i in 1..=n {
        arr[i] = scan.token();
    }

    let m = scan.token();

    let mut query = vec![(0, 0); m + 1];
    let mut block = vec![((0, 0), 0); m + 1];

    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=m {
        query[i] = (scan.token(), scan.token());
        block[i].0 = (query[i].0 / sqrt_n, query[i].1);
        block[i].1 = i;
    }

    block.sort();

    let mut count: Vec<usize> = vec![0; 1_000_001];
    let mut ans = vec![0; m + 1];
    let mut ans_query = 0;

    for i in 1..=m {
        let l1 = query[block[i - 1].1].0 as i32;
        let r1 = query[block[i - 1].1].1 as i32;
        let l2 = query[block[i].1].0 as i32;
        let r2 = query[block[i].1].1 as i32;

        for j in (l2..=(l1 - 1)).rev() {
            if count[arr[j as usize]] == 0 {
                ans_query += 1;
            }

            count[arr[j as usize]] += 1;
        }

        for j in (r1 + 1)..=r2 {
            if count[arr[j as usize]] == 0 {
                ans_query += 1;
            }

            count[arr[j as usize]] += 1;
        }

        for j in l1..l2 {
            count[arr[j as usize]] -= 1;

            if count[arr[j as usize]] == 0 {
                ans_query -= 1;
            }
        }

        for j in ((r2 + 1)..=r1).rev() {
            count[arr[j as usize]] -= 1;

            if count[arr[j as usize]] == 0 {
                ans_query -= 1;
            }
        }

        ans[block[i].1] = ans_query;
    }

    for i in 1..=m {
        writeln!(out, "{}", ans[i]).unwrap();
    }
}
