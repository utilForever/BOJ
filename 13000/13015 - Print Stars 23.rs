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

    let print1 = |out: &mut io::BufWriter<io::StdoutLock>| {
        for _ in 0..n {
            write!(out, "*").unwrap();
        }

        for _ in 0..(n - 1) * 2 - 1 {
            write!(out, " ").unwrap();
        }

        for _ in 0..n {
            write!(out, "*").unwrap();
        }

        writeln!(out).unwrap();
    };

    let print2 = |out: &mut io::BufWriter<io::StdoutLock>| {
        for i in 1..=n - 2 {
            for _ in 0..i {
                write!(out, " ").unwrap();
            }

            write!(out, "*").unwrap();

            for _ in 0..n - 2 {
                write!(out, " ").unwrap();
            }

            write!(out, "*").unwrap();

            for _ in 0..(n - 1 - i) * 2 - 1 {
                write!(out, " ").unwrap();
            }

            write!(out, "*").unwrap();

            for _ in 0..n - 2 {
                write!(out, " ").unwrap();
            }

            writeln!(out, "*").unwrap();
        }
    };

    let print2_rev = |out: &mut io::BufWriter<io::StdoutLock>| {
        for i in (1..=n - 2).rev() {
            for _ in 0..i {
                write!(out, " ").unwrap();
            }

            write!(out, "*").unwrap();

            for _ in 0..n - 2 {
                write!(out, " ").unwrap();
            }

            write!(out, "*").unwrap();

            for _ in 0..(n - 1 - i) * 2 - 1 {
                write!(out, " ").unwrap();
            }

            write!(out, "*").unwrap();

            for _ in 0..n - 2 {
                write!(out, " ").unwrap();
            }

            writeln!(out, "*").unwrap();
        }
    };

    let print3 = |out: &mut io::BufWriter<io::StdoutLock>| {
        for _ in 0..n - 1 {
            write!(out, " ").unwrap();
        }

        write!(out, "*").unwrap();

        for _ in 0..n - 2 {
            write!(out, " ").unwrap();
        }

        write!(out, "*").unwrap();

        for _ in 0..n - 2 {
            write!(out, " ").unwrap();
        }

        writeln!(out, "*").unwrap();
    };

    print1(&mut out);
    print2(&mut out);
    print3(&mut out);
    print2_rev(&mut out);
    print1(&mut out);
}
