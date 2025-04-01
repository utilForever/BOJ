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

    let (n, _) = (scan.token::<usize>(), scan.token::<i64>());
    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut partial = Vec::new();

    for i in 0..n {
        if s[i] == 'X' {
            continue;
        }

        for j in i + 1..n {
            if s[j] == 'X' {
                continue;
            }

            if s[i] != s[j] {
                continue;
            }

            partial.push((i, j));
        }
    }

    let mut ret = 0;

    for i in 0..partial.len() {
        ret = ret.max(partial[i].1 - partial[i].0 + 1);
    }

    for i in 0..partial.len() {
        for j in i + 1..partial.len() {
            if partial[i].1 >= partial[j].0 {
                continue;
            }

            let len1 = partial[i].1 - partial[i].0 + 1;
            let len2 = partial[j].1 - partial[j].0 + 1;
            ret = ret.max(len1 + len2);
        }
    }

    writeln!(out, "{}", n - ret).unwrap();
}
