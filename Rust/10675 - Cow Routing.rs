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

    let (a, b, n) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = -1;

    for _ in 0..n {
        let (cost, cnt_routes) = (scan.token::<i64>(), scan.token::<usize>());
        let mut routes = vec![0; cnt_routes];

        for i in 0..cnt_routes {
            routes[i] = scan.token::<i64>();
        }

        let pos_a = routes.iter().position(|&x| x == a);
        let pos_b = routes.iter().position(|&x| x == b);

        if pos_a.is_some() && pos_b.is_some() && pos_a.unwrap() < pos_b.unwrap() {
            if ret == -1 {
                ret = cost;
            } else {
                ret = ret.min(cost);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
