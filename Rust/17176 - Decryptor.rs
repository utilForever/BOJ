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

    let n = scan.token::<usize>();
    let mut password = vec![0; 53];

    for _ in 0..n {
        let idx = scan.token::<usize>();
        password[idx] += 1;
    }

    let plain = scan.line().chars().collect::<Vec<_>>();
    let mut encrypted = vec![0; 53];

    for c in plain {
        match c {
            ' ' => encrypted[0] += 1,
            'a'..='z' => encrypted[c as usize - 'a' as usize + 27] += 1,
            'A'..='Z' => encrypted[c as usize - 'A' as usize + 1] += 1,
            _ => (),
        }
    }

    writeln!(out, "{}", if password == encrypted { "y" } else { "n" }).unwrap();
}
