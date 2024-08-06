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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ranks = vec![-1; n];

    for i in 0..n {
        let rank = scan.token::<usize>();

        if ranks[rank - 1] == -1 {
            ranks[rank - 1] = i as i64;
        } else {
            for j in (rank - 1..n - 1).rev() {
                if ranks[j] == -1 {
                    continue;
                }

                ranks[j + 1] = ranks[j];
                ranks[j] = -1;
            }

            ranks[rank - 1] = i as i64;
        }
    }

    let ranks_next = ranks[..m].to_vec();
    let mut ret = vec![-1; m];

    for i in (0..m).rev() {
        let rank = scan.token::<usize>();

        if ret[rank - 1] == -1 {
            ret[rank - 1] = ranks_next[i];
        } else {
            for j in (rank - 1..m - 1).rev() {
                if ret[j] == -1 {
                    continue;
                }

                ret[j + 1] = ret[j];
                ret[j] = -1;
            }

            ret[rank - 1] = ranks_next[i];
        }
    }

    for i in 0..3 {
        writeln!(out, "{}", ret[i] + 1).unwrap();
    }
}
