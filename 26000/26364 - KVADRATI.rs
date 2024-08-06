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

    let n = scan.token::<usize>();
    let mut specials = [[false; 26]; 26];

    for _ in 0..n {
        let (a, b) = (
            (scan.token::<char>() as u8 - b'a') as usize,
            (scan.token::<char>() as u8 - b'a') as usize,
        );
        specials[a][b] = true;
        specials[b][a] = true;
    }

    let s = scan.token::<String>();
    let s = s
        .chars()
        .map(|c| (c as u8 - b'a') as usize)
        .collect::<Vec<_>>();
    let len = s.len();
    let mut ret = 0;

    for offset in 1..len {
        let mut cnt = 0;

        for i in 0..len - offset {
            if specials[s[i]][s[i + offset]] {
                cnt += 1;
            }

            if i >= offset && specials[s[i - offset]][s[i]] {
                cnt -= 1;
            }

            if cnt == offset {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
