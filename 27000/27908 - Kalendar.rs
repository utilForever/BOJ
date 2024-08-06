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

    let (n, d) = (scan.token::<i64>(), scan.token::<i64>());

    writeln!(out, "+---------------------+").unwrap();

    let start = 7 - d + 1;
    let rest = n - start;
    let row = if n + d <= 7 {
        0
    } else if rest % 7 == 0 {
        rest / 7
    } else {
        rest / 7 + 1
    };
    let mut day = 1;

    write!(out, "|").unwrap();

    for _ in 0..(7 - start) {
        write!(out, "...").unwrap();
    }

    for _ in (7 - start)..7 {
        if day > n {
            write!(out, "...").unwrap();
        } else if day < 10 {
            write!(out, "..{day}").unwrap();
        } else {
            write!(out, ".{day}").unwrap();
        }

        day += 1;
    }

    writeln!(out, "|").unwrap();

    for _ in 1..=row {
        write!(out, "|").unwrap();

        for _ in 1..=7 {
            if day > n {
                write!(out, "...").unwrap();
            } else if day < 10 {
                write!(out, "..{day}").unwrap();
            } else {
                write!(out, ".{day}").unwrap();
            }

            day += 1;
        }

        writeln!(out, "|").unwrap();
    }

    writeln!(out, "+---------------------+").unwrap();
}
