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

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
        let mine = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut mutations = vec![true; m];

        for _ in 0..n {
            let ancestors = scan.token::<String>().chars().collect::<Vec<_>>();

            for j in 0..m {
                if mine[j] == ancestors[j] {
                    mutations[j] = false;
                }
            }
        }

        writeln!(out, "Data Set {i}:").unwrap();
        writeln!(out, "{}/{m}", mutations.iter().filter(|&&x| x).count()).unwrap();

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
