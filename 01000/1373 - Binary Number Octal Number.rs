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
    let mut ret = String::new();
    let mut chars = s.chars().rev();

    while let Some(c) = chars.next() {
        let c = c as u8 - '0' as u8;
        let b = chars.next().unwrap_or('0') as u8 - '0' as u8;
        let a = chars.next().unwrap_or('0') as u8 - '0' as u8;

        ret.push_str(&format!("{}", a * 4 + b * 2 + c * 1));
    }

    ret.chars()
        .rev()
        .for_each(|c| out.write_all(c.to_string().as_bytes()).unwrap());
    out.write_all(b"\n").unwrap();
}
