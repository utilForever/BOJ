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

    let mut k = scan.token::<usize>() - 1;
    let mut tens = [0; 17];
    tens[0] = 1;

    for i in 1..17 {
        tens[i] = tens[i - 1] * 10;
    }

    let mut len = 1;

    loop {
        let cnt = 9 * tens[(len - 1) / 2] * len;

        if k < cnt {
            break;
        }

        k -= cnt;
        len += 1;
    }

    let mut pos = k / len + tens[(len - 1) / 2];
    let mut offset = k % len;

    if offset >= len / 2 {
        offset = len - offset - 1;
    }

    for _ in 0..(len + 1) / 2 - offset - 1 {
        pos /= 10;
    }

    writeln!(out, "{}", pos % 10).unwrap();
}
