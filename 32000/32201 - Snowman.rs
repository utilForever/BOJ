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

    let n = scan.token::<usize>();
    let mut before = HashMap::with_capacity(n);
    let mut after = HashMap::with_capacity(n);

    for i in 1..=n {
        let num = scan.token::<i64>();
        before.insert(num, i as i64);
    }

    for i in 1..=n {
        let num = scan.token::<i64>();
        after.insert(num, i as i64);
    }

    let mut diff = vec![(0, 0, 0); n];

    for (idx, (&num, &rank_before)) in before.iter().enumerate() {
        let rank_after = *after.get(&num).unwrap();
        let diff_rank = rank_before - rank_after;

        diff[idx] = (num, diff_rank, rank_after);
    }

    diff.sort_by(|a, b| b.1.cmp(&a.1));

    let change = diff[0].1;
    let mut ret = vec![(diff[0].0, diff[0].2)];

    for i in 1..n {
        if diff[i].1 == change {
            ret.push((diff[i].0, diff[i].2));
        } else {
            break;
        }
    }

    ret.sort_by(|a, b| a.1.cmp(&b.1));

    for val in ret {
        write!(out, "{} ", val.0).unwrap();
    }

    writeln!(out).unwrap();
}
