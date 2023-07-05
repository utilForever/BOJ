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
    let mut classes = vec![vec![0; 5]; n];
    let mut lectures = vec![vec![0; 5]; 5];

    for i in 0..n {
        for j in 0..5 {
            classes[i][j] = scan.token::<i64>();
        }
    }

    for i in 0..5 {
        for j in 0..5 {
            if i == j {
                continue;
            }

            for k in 0..n {
                if classes[k][i] == 1 && classes[k][j] == 1 {
                    lectures[i][j] += 1;
                }
            }
        }
    }

    let ret = lectures
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, v)| ((i, j), v))
                .max_by_key(|(_, v)| *v)
                .unwrap()
        })
        .max_by_key(|(_, v)| *v)
        .unwrap();

    writeln!(out, "{}", ret.1).unwrap();

    if *ret.1 == 0 {
        writeln!(out, "1 1 0 0 0").unwrap();
        return;
    }

    for i in 0..5 {
        write!(
            out,
            "{} ",
            if i == ret.0 .0 || i == ret.0 .1 { 1 } else { 0 }
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}
