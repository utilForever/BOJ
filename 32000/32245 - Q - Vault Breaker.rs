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

    let (n, b) = (scan.token::<usize>(), scan.token::<i64>());
    let mut prev = 0;
    let mut last = 0;

    for i in (0..n).rev() {
        let s = scan.token::<i64>();

        if i == n - 2 {
            prev = s;
        } else if i < n - 2 {
            if s != prev {
                prev = -1;
            }
        }

        if i == 0 {
            last = s;
        }
    }

    if n == 1 || prev != -1 {
        writeln!(out, "{b}").unwrap();

        let mut cnt = 0;

        while last < b - 1 {
            write!(out, "B").unwrap();

            last += 1;
            cnt += 1;
        }

        write!(out, "A").unwrap();
        cnt += 1;

        while cnt < b {
            write!(out, "B").unwrap();
            cnt += 1;
        }
    } else {
        writeln!(out, "{}", 2 * b).unwrap();

        let mut cnt_a = 0;

        for _ in 0..2 * b {
            if last == b - 1 {
                write!(out, "B").unwrap();
                last = 0;
            } else if cnt_a == b {
                write!(out, "B").unwrap();
                last += 1;
            } else {
                write!(out, "A").unwrap();
                cnt_a += 1;
                last += 1;
            }
        }
    }

    writeln!(out).unwrap();
}
