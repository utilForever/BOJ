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
 
    let (a, b, n) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
 
    if b / 10 == 2 || b / 10 == 4 || b / 10 == 5 || b / 10 == 6 || b / 10 == 8 {
        writeln!(out, "-1").unwrap();
        return;
    }
 
    let start = if a % 10 == 2 || a % 10 == 5 || a % 10 == 8 || a % 10 == 9 {
        7
    } else {
        1
    };
 
    write!(out, "{a}").unwrap();
    write!(out, "{start}").unwrap();
 
    for _ in 0..n - 5 {
        write!(out, "1").unwrap();
    }
    
    writeln!(out, "{b}").unwrap();
}
