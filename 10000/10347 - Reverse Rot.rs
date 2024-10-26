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

    let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_."
        .chars()
        .collect::<Vec<char>>();

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut s = scan.token::<String>().chars().collect::<Vec<_>>();

        for i in 0..s.len() {
            let curr = characters.iter().position(|&x| x == s[i]).unwrap();
            s[i] = characters[(curr + n) % characters.len()];
        }

        s.reverse();

        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
    }
}
