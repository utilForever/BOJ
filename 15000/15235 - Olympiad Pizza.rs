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
    let mut slices = vec![0; n];

    for i in 0..n {
        slices[i] = scan.token::<i64>();
    }

    for i in 0..n {
        let mut slices_clone = slices.clone();
        let mut idx = 0;
        let mut ret = 0;

        while slices_clone[i] > 0 {
            if slices_clone[idx] == 0 {
                idx = (idx + 1) % n;
                continue;
            }

            slices_clone[idx] -= 1;
            ret += 1;
            idx = (idx + 1) % n;
        }

        write!(out, "{ret} ").unwrap();
    }

    writeln!(out).unwrap();
}
