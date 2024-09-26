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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MAX: i64 = 2i64.pow(60) - 1;

fn check_tree_blocking(set: &BTreeSet<i64>, cnt: &mut i64, node: i64) -> bool {
    if set.contains(&node) {
        return true;
    }

    *cnt += 1;

    if node > MAX {
        return false;
    }

    if !check_tree_blocking(set, cnt, node * 2) {
        return false;
    }

    if !check_tree_blocking(set, cnt, node * 2 + 1) {
        return false;
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut set = BTreeSet::new();

    for _ in 0..n {
        let v = scan.token::<i64>();
        set.insert(v);
    }

    let mut cnt = 0;
    let ret = check_tree_blocking(&set, &mut cnt, 1);

    writeln!(out, "{}", if ret { cnt } else { -1 }).unwrap();
}
