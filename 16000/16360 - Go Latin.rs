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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let mut word = scan.token::<String>();

        if word.ends_with("a") {
            word.pop();
            word.push_str("as");
        } else if word.ends_with("i") || word.ends_with("y") {
            word.pop();
            word.push_str("ios");
        } else if word.ends_with("l") {
            word.pop();
            word.push_str("les");
        } else if word.ends_with("n") || word.ends_with("ne") {
            if word.ends_with("n") {
                word.pop();
            } else {
                word.pop();
                word.pop();
            }

            word.push_str("anes");
        } else if word.ends_with("o") {
            word.pop();
            word.push_str("os");
        } else if word.ends_with("r") {
            word.pop();
            word.push_str("res");
        } else if word.ends_with("t") {
            word.pop();
            word.push_str("tas");
        } else if word.ends_with("u") {
            word.pop();
            word.push_str("us");
        } else if word.ends_with("v") {
            word.pop();
            word.push_str("ves");
        } else if word.ends_with("w") {
            word.pop();
            word.push_str("was");
        } else {
            word.push_str("us");
        }

        writeln!(out, "{word}").unwrap();
    }
}
