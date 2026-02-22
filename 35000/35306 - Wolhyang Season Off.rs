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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut max_index = vec![-1; k];
    let mut max_cnt = vec![0; k];
    let mut idx_people = vec![None; k];

    for i in 0..n {
        for j in 0..k {
            let index = scan.token::<i64>();

            if index > max_index[j] {
                max_index[j] = index;
                max_cnt[j] = 1;
                idx_people[j] = Some(i);
            } else if index == max_index[j] {
                max_cnt[j] += 1;
                idx_people[j] = None;
            }
        }
    }

    let mut ret = vec![false; n];

    for i in 0..k {
        if max_cnt[i] == 1 {
            ret[idx_people[i].unwrap()] = true;
        }
    }

    writeln!(out, "{}", ret.iter().filter(|&&x| x).count()).unwrap();
}
