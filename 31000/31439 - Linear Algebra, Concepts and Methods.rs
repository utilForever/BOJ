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

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return true;
    }

    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    if n == 2 {
        writeln!(out, "YES").unwrap();
        writeln!(out, "1 2").unwrap();
        return;
    }

    if n > 2 && n < 10 {
        writeln!(out, "NO").unwrap();
        return;
    }

    if is_prime(n - 1) {
        writeln!(out, "NO").unwrap();
        return;
    }

    writeln!(out, "YES").unwrap();

    if n % 2 == 0 {
        for i in (n % 4 + 2..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in ((n + 2) % 4 + 2..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (1..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (3..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }
    } else if n % 6 == 1 {
        for i in ((n + 1) % 4 + 2..=n - 2).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in ((n - 1) % 4 + 2..=n - 2).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        write!(out, "{} ", 1).unwrap();

        for i in (2..=n - 2).rev().step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (2..=n - 4).rev().step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        write!(out, "{} ", n).unwrap();
        write!(out, "{} ", n - 1).unwrap();
    } else if n % 6 == 3 {
        for i in (10..=n - 6).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (4..=n - 6).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        write!(out, "{} ", 2).unwrap();
        write!(out, "{} ", n - 5).unwrap();
        write!(out, "{} ", 1).unwrap();

        for i in (2..=n - 2).rev().step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (2..=n - 4).rev().step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        write!(out, "{} ", n).unwrap();
        write!(out, "{} ", n - 1).unwrap();
        write!(out, "{} ", 6).unwrap();
        write!(out, "{} ", n - 3).unwrap();
    } else {
        for i in ((n - 1) % 4 + 2..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in ((n + 1) % 4 + 2..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (1..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }

        for i in (3..=n).step_by(4) {
            write!(out, "{i} ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
