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
    let mut ret = [(0, 0); 4];
    let mut gomgoms = Vec::new();

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            if c == 'G' {
                gomgoms.push((i, j));

                ret[0] = (ret[0].0.max(i), ret[0].1.max(j));
                ret[1] = (ret[1].0.max(n - i - 1), ret[1].1.max(j));
                ret[2] = (ret[2].0.max(i), ret[2].1.max(n - j - 1));
                ret[3] = (ret[3].0.max(n - i - 1), ret[3].1.max(n - j - 1));
            }
        }
    }

    if gomgoms.len() == 1 {
        writeln!(out, "0").unwrap();
    } else if gomgoms.len() <= n {
        if gomgoms.iter().all(|(i, _)| *i == gomgoms[0].0) {
            let vec = gomgoms.iter().map(|(_, j)| *j).collect::<Vec<_>>();
            let left = *vec.iter().max().unwrap();
            let right = (n - 1) - *vec.iter().min().unwrap();

            writeln!(out, "{}", left.min(right)).unwrap();
        } else if gomgoms.iter().all(|(_, j)| *j == gomgoms[0].1) {
            let vec = gomgoms.iter().map(|(i, _)| *i).collect::<Vec<_>>();
            let up = *vec.iter().max().unwrap();
            let down = (n - 1) - *vec.iter().min().unwrap();

            writeln!(out, "{}", up.min(down)).unwrap();
        } else {
            writeln!(out, "{}", ret.iter().map(|(x, y)| x + y).min().unwrap()).unwrap();
        }
    } else {
        writeln!(out, "{}", ret.iter().map(|(x, y)| x + y).min().unwrap()).unwrap();
    }
}
