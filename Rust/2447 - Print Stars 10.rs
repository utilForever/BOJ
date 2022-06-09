use io::Write;
use std::{io::{self, BufWriter, StdoutLock}, str};

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

fn print_stars(out: &mut BufWriter<StdoutLock>, i: usize, j: usize, n: usize) {
    if (i / n) % 3 == 1 && (j / n) % 3 == 1 {
        write!(out, " ").unwrap();
    } else {
        if n / 3 == 0 {
            write!(out, "*").unwrap();
        } else {
            print_stars(out, i, j, n / 3);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    for i in 0..n {
        for j in 0..n {
            print_stars(&mut out, i, j, n);
        }

        writeln!(out, "").unwrap();
    }
}
