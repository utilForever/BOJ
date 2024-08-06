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

    let (s1, s2) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret_sample = true;
    let mut ret_system = true;

    for _ in 0..s1 {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        if a != b {
            ret_sample = false;
        }
    }

    for _ in 0..s2 {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        if a != b {
            ret_system = false;
        }
    }

    writeln!(
        out,
        "{}",
        if ret_sample && ret_system {
            "Accepted"
        } else if ret_sample {
            "Why Wrong!!!"
        } else {
            "Wrong Answer"
        }
    )
    .unwrap();
}
