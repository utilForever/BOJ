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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn clear(&mut self) {
        for val in self.data.iter_mut() {
            *val = 0;
        }
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        while idx <= self.n {
            if self.data[idx] < val {
                self.data[idx] = val;
            }

            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret = ret.max(self.data[idx]);
            idx -= idx & (!idx + 1);
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut perm = vec![0; n];

    for i in 0..n {
        perm[i] = scan.token::<usize>();
    }

    let mut partner = vec![0; n + 1];

    for i in 1..=n {
        let j = perm[i - 1];

        if i < j {
            partner[i] = j;
        }
    }

    let mut clique_max = 0;
    let mut fenwick = FenwickTree::new(n);

    for s in 1..=n {
        let e = partner[s];

        if e == 0 {
            continue;
        }

        fenwick.clear();

        let mut clique_cnt = 0;

        for i in (s + 1)..e {
            let r = partner[i];

            if r > e {
                let curr = fenwick.query(r - 1) + 1;

                fenwick.update(r, curr);
                clique_cnt = clique_cnt.max(curr);
            }
        }

        clique_max = clique_max.max(clique_cnt + 1);
    }

    let ret = (clique_max as usize + k).min(n / 2);

    writeln!(out, "{ret}").unwrap();
}
