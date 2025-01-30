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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut ratings = vec![0; n];
    let mut ret_cnt = 0;
    let mut ret_max = 0;

    for i in 0..n {
        ratings[i] = scan.token::<i64>();
    }

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let min = ratings[i].min(ratings[j]).min(ratings[k]);
                let max = ratings[i].max(ratings[j]).max(ratings[k]);

                if max - min <= m {
                    let sum = ratings[i] + ratings[j] + ratings[k];

                    ret_cnt += 1;
                    ret_max = ret_max.max(sum);
                }
            }
        }
    }

    if ret_cnt == 0 {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{ret_cnt} {ret_max}").unwrap();
    }
}
