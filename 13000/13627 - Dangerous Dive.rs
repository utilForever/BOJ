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

    let (n, r) = (scan.token::<usize>(), scan.token::<usize>());
    let mut volunteers = vec![false; n];

    for _ in 0..r {
        let volunteer = scan.token::<usize>();
        volunteers[volunteer - 1] = true;
    }

    if volunteers.iter().all(|&v| v) {
        writeln!(out, "*").unwrap();
    } else {
        for (i, &is_returned) in volunteers.iter().enumerate() {
            if is_returned {
                continue;
            }

            write!(out, "{} ", i + 1).unwrap();
        }

        writeln!(out).unwrap();
    }
}
