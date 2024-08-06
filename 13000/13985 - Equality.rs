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

    let a = scan.token::<String>();
    _ = scan.token::<String>();
    let b = scan.token::<String>();
    _ = scan.token::<String>();
    let c = scan.token::<String>();

    let a = (a.chars().next().unwrap() as u32 - '0' as u32) as i64;
    let b = (b.chars().next().unwrap() as u32 - '0' as u32) as i64;
    let c = (c.chars().next().unwrap() as u32 - '0' as u32) as i64;

    writeln!(out, "{}", if a + b == c { "YES" } else { "NO" }).unwrap();
}
