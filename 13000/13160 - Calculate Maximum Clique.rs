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

    let n = scan.token::<i64>();
    let mut positions = Vec::new();
    let mut vertices = Vec::new();

    for i in 0..n {
        let (s, e) = (scan.token::<i64>(), scan.token::<i64>());

        positions.push((s, e));
        vertices.push((s, 1));
        vertices.push((e, 2));
    }

    vertices.sort();

    let mut cnt = 0;
    let mut ret = 0;
    let mut pos = 0;

    for vertice in vertices.iter() {
        if vertice.1 == 1 {
            cnt += 1;
        } else {
            cnt -= 1;
        }

        if cnt > ret {
            ret = cnt;
            pos = vertice.0;
        }
    }

    writeln!(out, "{ret}").unwrap();

    for (idx, position) in positions.iter().enumerate() {
        if position.0 <= pos && pos <= position.1 {
            write!(out, "{} ", idx + 1).unwrap();
        }
    }

    writeln!(out).unwrap();
}
