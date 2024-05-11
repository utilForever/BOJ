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

    let mut ret1 = vec![0; 3];
    let mut ret2 = vec![(0, String::new()); 3];

    for i in 0..3 {
        let (p, y, s) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<String>(),
        );
        ret1[i] = y % 100;
        ret2[i] = (p, s);
    }

    ret1.sort();
    ret2.sort_by(|a, b| b.0.cmp(&a.0));

    for val in ret1 {
        write!(out, "{val}").unwrap();
    }

    writeln!(out).unwrap();

    for (_, val) in ret2 {
        write!(out, "{}", val.chars().next().unwrap()).unwrap();
    }

    writeln!(out).unwrap();
}
