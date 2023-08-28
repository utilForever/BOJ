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
}

fn prev(set: &BTreeSet<(i64, i64)>, value: (i64, i64)) -> (i64, i64) {
    *set.range(..value).nth_back(0).unwrap()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<usize>();
    let mut set = BTreeSet::new();
    let mut ret = 0;

    set.insert((0, 1_000_000_001));
    set.insert((1_000_000_001, 0));

    for _ in 0..q {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let iter = *set.range((x, y)..).next().unwrap();

        if iter.1 >= y {
            writeln!(out, "{ret}").unwrap();
            continue;
        }

        ret -= (iter.0 - prev(&set, iter).0) * iter.1;

        loop {
            let p = prev(&set, iter).clone();

            if p.1 > y {
                break;
            }

            ret -= (p.0 - prev(&set, p).0) * p.1;
            set.remove(&p);
        }

        ret += (x - prev(&set, iter).0) * y;
        ret += (iter.0 - x) * iter.1;

        set.insert((x, y));

        writeln!(out, "{ret}").unwrap();
    }
}
