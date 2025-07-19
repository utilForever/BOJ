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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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
    let s = scan.line().trim().to_string();
    let mut splitted = Vec::new();

    for word in s.split_whitespace() {
        splitted.push(word.to_string());
    }

    let mut pos = 0;

    for i in 0..splitted.len() {
        if pos == 0 {
            write!(out, "{}", splitted[i]).unwrap();
            pos += splitted[i].len();
        } else {
            if pos + 1 + splitted[i].len() <= n {
                write!(out, " {}", splitted[i]).unwrap();
                pos += 1 + splitted[i].len();
            } else {
                writeln!(out).unwrap();
                write!(out, "{}", splitted[i]).unwrap();
                pos = splitted[i].len();
            }
        }
    }

    writeln!(out).unwrap();
}
