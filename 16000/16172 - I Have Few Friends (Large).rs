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
    let s = s
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<String>();
    let k = scan.token::<String>();

    let s_chars = s.as_bytes();
    let s_len = s_chars.len();
    let k_chars = k.as_bytes();
    let k_len = k_chars.len();

    let mut cmp = 0;
    let mut fail = vec![0; 200_000];

    for i in 1..k_len {
        while cmp > 0 && k_chars[cmp] != k_chars[i] {
            cmp = fail[cmp - 1];
        }

        if k_chars[cmp] == k_chars[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    cmp = 0;

    for i in 0..s_len {
        while cmp > 0 && s_chars[i] != k_chars[cmp] {
            cmp = fail[cmp - 1];
        }

        if s_chars[i] == k_chars[cmp] {
            if cmp == k_len - 1 {
                writeln!(out, "1").unwrap();
                return;
            } else {
                cmp += 1;
            }
        }
    }

    writeln!(out, "0").unwrap();
}
