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

    let (n, _) = (scan.token::<i64>(), scan.token::<i64>());
    let mut map = HashMap::new();

    for i in 1..=n {
        let num = scan.token::<i64>();
        map.entry(num).or_insert((0, i)).0 += 1;
    }

    let mut nums = map.iter().map(|(k, v)| (*k, v.0, v.1)).collect::<Vec<_>>();
    nums.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));

    for (num, _, _) in nums {
        for _ in 0..map[&num].0 {
            write!(out, "{num} ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
