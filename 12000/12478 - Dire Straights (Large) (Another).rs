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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut cnt = vec![0; 10001];

        for _ in 0..n {
            let card = scan.token::<usize>();
            cnt[card] += 1;
        }

        let mut straights = Vec::new();
        let mut remain = n;

        loop {
            let mut idx = 1;

            while idx <= 10000 {
                if cnt[idx] == 0 {
                    idx += 1;
                    continue;
                }

                let start = idx;

                while idx <= 10000 && cnt[idx] > 0 {
                    cnt[idx] -= 1;
                    idx += 1;
                }

                straights.push((start, idx - 1));
                remain -= idx - start;
            }

            if remain == 0 {
                break;
            }
        }

        loop {
            let mut changed = false;

            for i in 0..straights.len() {
                for j in i + 1..straights.len() {
                    let (s1, e1) = straights[i];
                    let (s2, e2) = straights[j];

                    if s2 > s1 && e2 < e1 {
                        straights[i] = (s1, e2);
                        straights[j] = (s2, e1);
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }

            straights.sort_unstable();
        }

        let mut ret = i64::MAX;

        while let Some((s, e)) = straights.pop() {
            ret = ret.min((e - s + 1) as i64);
        }

        writeln!(out, "Case #{i}: {}", if ret == i64::MAX { 0 } else { ret }).unwrap();
    }
}
