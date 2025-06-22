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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut prefix_sum = vec![vec![0; s.len() + 1]; s.len() + 1];

    for i in 0..s.len() {
        prefix_sum[i][i] = (s[i] as u8 - b'0') as i64;

        for j in i + 1..s.len() {
            prefix_sum[i][j] = prefix_sum[i][j - 1] + (s[j] as u8 - b'0') as i64;
        }
    }

    let mut ret = 0;

    for i in 0..s.len() {
        let mut idx = i;

        for j in (i + 1..s.len()).step_by(2) {
            if prefix_sum[i][j] % 2 == 0 && prefix_sum[i][j] / 2 == prefix_sum[i][idx] {
                ret = ret.max(j - i + 1);
            }

            idx += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
