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

    let n = scan.token::<usize>();
    let mut buses = vec![(0, 0, 0); n];

    for i in 0..n {
        buses[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    buses.sort();

    let mut ret = Vec::new();
    let mut curr = buses[0];

    for i in 1..n {
        let next = buses[i];

        if curr.1 >= next.0 {
            curr.1 = curr.1.max(next.1);
            curr.2 = curr.2.min(next.2);
        } else {
            ret.push(curr);
            curr = next;
        }
    }

    ret.push(curr);

    writeln!(out, "{}", ret.len()).unwrap();

    for (s, e, c) in ret {
        writeln!(out, "{s} {e} {c}").unwrap();
    }
}
