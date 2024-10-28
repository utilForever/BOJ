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

fn check(trees: &Vec<usize>, cut_height: usize, minimum_tree_len: usize) -> bool {
    let mut cnt = 0;

    for i in 0..trees.len() {
        let remain_height = trees[i] as i64 - cut_height as i64;

        if remain_height > 0 {
            cnt += remain_height;
        }
    }

    cnt >= minimum_tree_len as i64
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut trees = vec![0; n];

    for i in 0..n {
        trees[i] = scan.token::<usize>();
    }

    let mut l = 1;
    let mut r = *trees.iter().max().unwrap();
    let mut ans = 0;

    while l <= r {
        let mid = (l + r) / 2;

        if check(&trees, mid, m) {
            if ans < mid {
                ans = mid;
            }

            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
