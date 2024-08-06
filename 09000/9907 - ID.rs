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

    let digits = scan.token::<String>();
    let digits = digits.chars().collect::<Vec<_>>();
    let weights = [2, 7, 6, 5, 4, 3, 2];
    let letters = ['J', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'Z'];
    let mut ret = 0;

    for (i, c) in digits.iter().enumerate() {
        let digit = c.to_digit(10).unwrap();
        ret += digit * weights[i];
    }

    ret %= 11;

    writeln!(out, "{}", letters[ret as usize]).unwrap();
}
