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

    let (t1, v1) = (scan.token::<i32>(), scan.token::<i32>());
    let (t2, v2) = (scan.token::<i32>(), scan.token::<i32>());

    writeln!(
        out,
        "{}",
        if t2 < 0 && v2 >= 10 {
            "A storm warning for tomorrow! Be careful and stay home if possible!"
        } else if t1 > t2 {
            "MCHS warns! Low temperature is expected tomorrow."
        } else if v1 < v2 {
            "MCHS warns! Strong wind is expected tomorrow."
        } else {
            "No message"
        }
    )
    .unwrap();
}
