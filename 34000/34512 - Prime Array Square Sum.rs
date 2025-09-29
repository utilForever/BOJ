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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n == 2 {
        writeln!(out, "-1").unwrap();
        return;
    }

    if n == 3 {
        writeln!(out, "2 5 29").unwrap();
        return;
    }

    if n == 4 {
        writeln!(out, "2 2 3 17").unwrap();
        return;
    }

    if n == 5 {
        writeln!(out, "2 2 2 3 3").unwrap();
        return;
    }

    if n == 6 {
        writeln!(out, "2 2 3 3 3 5").unwrap();
        return;
    }

    let (x, y) = match n % 3 {
        0 => (n - 6, 6),
        1 => (n - 4, 4),
        2 => (n - 2, 2),
        _ => unreachable!(),
    };

    for _ in 0..x {
        write!(out, "2 ").unwrap();
    }

    for _ in 0..y {
        write!(out, "3 ").unwrap();
    }

    writeln!(out).unwrap();
}
