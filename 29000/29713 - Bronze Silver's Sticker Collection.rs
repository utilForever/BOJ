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

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();
    let mut alphabet = [0; 26];

    for c in s.chars() {
        alphabet[(c as u8 - b'A') as usize] += 1;
    }

    let b = alphabet[b'B' as usize - b'A' as usize];
    let r = alphabet[b'R' as usize - b'A' as usize] / 2;
    let o = alphabet[b'O' as usize - b'A' as usize];
    let n = alphabet[b'N' as usize - b'A' as usize];
    let z = alphabet[b'Z' as usize - b'A' as usize];
    let e = alphabet[b'E' as usize - b'A' as usize] / 2;
    let s = alphabet[b'S' as usize - b'A' as usize];
    let i = alphabet[b'I' as usize - b'A' as usize];
    let l = alphabet[b'L' as usize - b'A' as usize];
    let v = alphabet[b'V' as usize - b'A' as usize];

    writeln!(
        out,
        "{}",
        vec![b, r, o, n, z, e, r, o, s, i, s, l, i, v]
            .iter()
            .min()
            .unwrap()
    )
    .unwrap();
}
