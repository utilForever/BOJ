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

    let s = scan.token::<String>();
    let mut s = s.chars().collect::<Vec<_>>();

    let mut first_u = None;
    let mut last_f = None;

    for (idx, ch) in s.iter().enumerate() {
        if *ch == 'U' && first_u == None {
            first_u = Some(idx);
        }

        if *ch == 'F' {
            last_f = Some(idx);
        }
    }

    for (idx, ch) in s.iter_mut().enumerate() {
        if idx < first_u.unwrap() {
            *ch = '-';
        } else if idx > first_u.unwrap() && idx < last_f.unwrap() {
            *ch = 'C';
        } else if idx > last_f.unwrap() {
            *ch = '-';
        }
    }

    writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
}
