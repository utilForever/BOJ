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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut alphabet = [0; 26];
        let mut idx = 0;
        let mut ret = true;

        while idx < s.len() {
            let pos = s[idx] as usize - 'A' as usize;
            alphabet[pos] += 1;

            if alphabet[pos] % 3 == 0 {
                if idx + 1 >= s.len() || s[idx + 1] != s[idx] {
                    ret = false;
                    break;
                }

                idx += 1;
            }

            idx += 1;
        }

        writeln!(out, "{}", if ret { "OK" } else { "FAKE" }).unwrap();
    }
}
