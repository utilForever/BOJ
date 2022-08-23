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
    let s = s.chars().collect::<Vec<char>>();
    let mut flag_l = 0;
    let mut flag_s = 0;
    let mut flag = false;
    let mut ret = 0;

    for c in s {
        if flag {
            break;
        }

        match c {
            'L' => {
                flag_l += 1;
            }
            'R' => {
                if flag_l > 0 {
                    ret += 1;
                    flag_l -= 1;
                } else {
                    flag = true;
                }
            }
            'S' => {
                flag_s += 1;
            }
            'K' => {
                if flag_s > 0 {
                    ret += 1;
                    flag_s -= 1;
                } else {
                    flag = true;
                }
            }
            _ => ret += 1,
        }
    }

    writeln!(out, "{ret}").unwrap();
}
