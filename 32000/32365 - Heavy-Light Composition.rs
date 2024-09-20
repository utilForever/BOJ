use io::Write;
use std::{io, str, vec};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (t, _) = (scan.token::<i64>(), scan.token::<i64>());

    for _ in 0..t {
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut alphabets = vec![0; 26];

        for &c in s.iter() {
            alphabets[(c as u8 - b'a') as usize] += 1;
        }

        let is_heavy_first = alphabets[(s[0] as u8 - b'a') as usize] > 1;
        let mut ret = true;

        for i in 0..s.len() {
            let should_heavy = if is_heavy_first {
                i % 2 == 0
            } else {
                i % 2 != 0
            };
            let idx = (s[i] as u8 - b'a') as usize;

            if (should_heavy && alphabets[idx] == 1) || (!should_heavy && alphabets[idx] != 1) {
                ret = false;
                break;
            }
        }

        writeln!(out, "{}", if ret { "T" } else { "F" }).unwrap();
    }
}
