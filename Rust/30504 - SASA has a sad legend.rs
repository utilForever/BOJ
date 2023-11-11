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
    let mut money_sejong = vec![(0, 0); n];
    let mut money_youngjae = vec![0; n];

    for i in 0..n {
        money_sejong[i] = (scan.token::<i64>(), i);
    }

    for i in 0..n {
        money_youngjae[i] = scan.token::<i64>();
    }

    money_sejong.sort();
    money_youngjae.sort();

    if money_sejong
        .iter()
        .zip(money_youngjae.iter())
        .all(|(a, b)| &a.0 <= b)
    {
        let mut ret = vec![0; n];

        for i in 0..n {
            ret[money_sejong[i].1] = money_youngjae[i];
        }

        for money in ret {
            write!(out, "{money} ").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
