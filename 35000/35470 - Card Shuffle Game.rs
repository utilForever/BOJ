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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();
    let mut positions = vec![0; n];

    for i in 0..n {
        positions[i] = scan.token::<usize>() - 1;
    }

    let mut target = s.chars().collect::<Vec<_>>();
    target.sort_unstable();

    let mut visited = vec![false; n];
    let mut ret = vec![' '; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut cycle = Vec::new();
        let mut idx = i;

        while !visited[idx] {
            visited[idx] = true;
            cycle.push(idx);
            idx = positions[idx];
        }

        for j in 0..cycle.len() {
            ret[cycle[j]] = target[cycle[(j + k) % cycle.len()]];
        }
    }

    writeln!(out, "{}", ret.into_iter().collect::<String>()).unwrap();
}
