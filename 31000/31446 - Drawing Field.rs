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

static MAX: i64 = 1_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, s) = (scan.token::<i64>(), scan.token::<i64>());

    if n - 2 > s {
        writeln!(out, "NO").unwrap();
        return;
    }

    writeln!(out, "YES").unwrap();

    if n == 3 {
        writeln!(out, "0 0").unwrap();
        writeln!(out, "{s} 0").unwrap();
        writeln!(out, "0 1").unwrap();
        return;
    }

    let mut ret = Vec::new();
    let mut cnt = 0;

    if n % 2 == 0 {
        ret.push((MAX - 1, 1));
        ret.push((MAX - 1, 0));
        ret.push((MAX, 2));

        cnt += 3;
    } else {
        ret.push((MAX - 1, 1));
        ret.push((MAX - 1, 0));
        ret.push((MAX, 1));
        ret.push((MAX, 2));

        cnt += 4;
    }

    let mut flag = 0;

    while cnt + 2 < n {
        if flag == 0 {
            ret.push((ret[0].0, ret[0].1 + 1));
            ret.insert(0, (ret[0].0 - 1, ret[0].1));
        } else {
            ret.push((ret[0].0 + 1, ret[0].1 + 2));
            ret.insert(0, (ret[0].0, ret[0].1 + 1));
        }

        cnt += 2;
        flag = 1 - flag;
    }

    if flag == 0 {
        ret.insert(0, (ret[0].0 - (s - cnt + 2), ret[0].1));
    } else {
        ret.insert(0, (ret[0].0, ret[0].1 + (s - cnt + 2)));
    }

    for point in ret {
        writeln!(out, "{} {}", point.0, point.1).unwrap();
    }
}
