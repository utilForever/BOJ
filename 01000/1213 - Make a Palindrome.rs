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
    let mut alphabets = [0; 26];

    for c in s.chars() {
        alphabets[(c as u8 - b'A') as usize] += 1;
    }

    if (s.len() % 2 == 1 && alphabets.iter().filter(|&&x| x % 2 == 1).count() != 1)
        || (s.len() % 2 == 0 && alphabets.iter().filter(|&&x| x % 2 == 1).count() != 0)
    {
        writeln!(out, "I'm Sorry Hansoo").unwrap();
        return;
    }

    let mut ret = String::new();

    for i in (0..26).rev() {
        while alphabets[i] > 1 {
            ret.insert(0, (i as u8 + b'A') as char);
            ret.push((i as u8 + b'A') as char);
            alphabets[i] -= 2;
        }
    }

    if s.len() % 2 == 1 {
        let pos = alphabets.iter().position(|&x| x % 2 == 1).unwrap();
        ret.insert(s.len() / 2, (pos as u8 + b'A') as char);
    }

    writeln!(out, "{ret}").unwrap();
}
