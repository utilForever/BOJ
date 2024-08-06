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
    let mut times_ad = vec![vec![0; 1440]; n];
    let mut ret = vec![0; n];

    for i in 0..n {
        let r = scan.token::<i64>();

        for _ in 0..r {
            let s = scan.token::<String>();
            let (hs, ms, he, me) = (
                s[0..2].parse::<usize>().unwrap(),
                s[3..5].parse::<usize>().unwrap(),
                s[6..8].parse::<usize>().unwrap(),
                s[9..11].parse::<usize>().unwrap(),
            );

            for j in hs * 60 + ms..=he * 60 + me {
                times_ad[i][j] = 1;
            }
        }
    }

    let mut idx = 0;

    for i in 0..1440 {
        if times_ad[idx][i] == 0 {
            ret[idx] += 1;
        } else {
            idx = (idx + 1) % n;
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
