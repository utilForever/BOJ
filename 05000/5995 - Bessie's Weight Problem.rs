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

    let (h, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut items = Vec::new();
    let mut ret = vec![vec![0; h + 1]; n + 1];

    for _ in 0..n {
        let w = scan.token::<u16>();
        items.push(w);
    }

    for i in 0..n {
        for j in 0..=h {
            ret[i][j] = if i == 0 {
                if items[i] <= j as u16 {
                    items[i]
                } else {
                    ret[i][j]
                }
            } else {
                if items[i] <= j as u16 {
                    ret[i - 1][j].max(ret[i - 1][j - items[i] as usize] + items[i])
                } else {
                    ret[i - 1][j]
                }
            }
        }
    }

    writeln!(out, "{}", ret[n - 1][h]).unwrap();
}
