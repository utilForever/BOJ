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
    let b = scan.token::<String>();
    let mut alphabet_a = [0; 26];
    let mut alphabet_b = [0; 26];

    for c in a.chars() {
        alphabet_a[(c as u8 - b'a') as usize] += 1;
    }

    for c in b.chars() {
        alphabet_b[(c as u8 - b'a') as usize] += 1;
    }

    let mut ret = [0; 26];

    for i in 0..26 {
        ret[i] = alphabet_a[i].max(alphabet_b[i]);
    }

    for i in 0..26 {
        for _ in 0..ret[i] {
            write!(out, "{}", (i as u8 + b'a') as char).unwrap();
        }
    }

    writeln!(out).unwrap();
}
