use io::Write;
use std::{cmp, io, str};

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

    let z = scan.token::<i32>();

    for _ in 0..z {
        let (w, k) = (scan.token::<i32>(), scan.token::<i32>());

        writeln!(
            out,
            "{}",
            if w % 2 == 0 && k % 2 == 0 {
                w / 2 * k
            } else if w % 2 == 0 {
                w / 2 * k
            } else if k % 2 == 0 {
                k / 2 * w
            } else {
                cmp::min(w, k) / 2 * cmp::max(w, k) + cmp::max(w, k) / 2
            }
        )
        .unwrap();
    }
}
