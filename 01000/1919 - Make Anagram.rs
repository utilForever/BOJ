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
    let s = s.chars().collect::<Vec<_>>();
    let t = scan.token::<String>();
    let t = t.chars().collect::<Vec<_>>();

    let mut alphabet_s = vec![0_i64; 26];
    let mut alphabet_t = vec![0_i64; 26];

    for c in s {
        alphabet_s[c as usize - 'a' as usize] += 1;
    }

    for c in t {
        alphabet_t[c as usize - 'a' as usize] += 1;
    }

    let mut ret = 0;

    for i in 0..26 {
        ret += (alphabet_s[i] - alphabet_t[i]).abs();
    }

    writeln!(out, "{ret}").unwrap();
}
