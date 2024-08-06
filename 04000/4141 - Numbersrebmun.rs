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
        let mut s = scan.token::<String>().chars().collect::<Vec<_>>();
        s.iter_mut().for_each(|c| *c = c.to_ascii_uppercase());

        for i in 0..s.len() {
            s[i] = match s[i] {
                'A' | 'B' | 'C' => '2',
                'D' | 'E' | 'F' => '3',
                'G' | 'H' | 'I' => '4',
                'J' | 'K' | 'L' => '5',
                'M' | 'N' | 'O' => '6',
                'P' | 'Q' | 'R' | 'S' => '7',
                'T' | 'U' | 'V' => '8',
                'W' | 'X' | 'Y' | 'Z' => '9',
                _ => unreachable!(),
            }
        }

        // Check it is palindrome
        let mut ret = true;

        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                ret = false;
                break;
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
