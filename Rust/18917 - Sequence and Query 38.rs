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

    let m = scan.token::<usize>();
    let mut val_sum = 0;
    let mut val_xor = 0;

    for _ in 0..m {
        let command = scan.token::<usize>();

        if command == 1 {
            let x = scan.token::<usize>();

            val_sum += x;
            val_xor ^= x;
        } else if command == 2 {
            let x = scan.token::<usize>();

            val_sum -= x;
            val_xor ^= x;
        } else if command == 3 {
            writeln!(out, "{}", val_sum).unwrap();
        } else {
            writeln!(out, "{}", val_xor).unwrap();
        }
    }
}
