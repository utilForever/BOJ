use std::{cmp, io, str};

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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token();
    let mut ret = 0;

    for _ in 0..n {
        let (mut x, mut y, c): (usize, usize, char) = (scan.token(), scan.token(), scan.token());

        if y > x {
            std::mem::swap(&mut x, &mut y);
        }

        if c == 'R' {
            ret ^= x ^ y;
        } else if c == 'B' {
            ret ^= cmp::min(x, y);
        } else if c == 'K' {
            if y % 2 == 0 {
                ret ^= if x % 2 == 0 { 0 } else { 1 };
            } else {
                ret ^= if x % 2 == 0 { 3 } else { 2 };
            }
        } else if c == 'N' {
            if x == y {
                ret ^= if x % 3 == 2 { 1 } else { 0 };
            } else if x == y + 1 {
                ret ^= if x % 3 == 0 { 0 } else { 1 };
            } else {
                ret ^= y % 3;
            }
        } else if c == 'P' {
            ret ^= ((x / 3) ^ (y / 3)) * 3 + (x + y) % 3;
        }
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
