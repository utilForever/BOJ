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

    let (mode, s) = (
        scan.token::<char>(),
        scan.token::<String>().chars().collect::<Vec<_>>(),
    );
    let mut ret = String::new();

    if mode == 'E' {
        let mut letter = s[0];
        let mut cnt = 1;

        for idx in 1..s.len() {
            if s[idx] == letter {
                cnt += 1;
            } else {
                ret.push(letter);
                ret.push(cnt.to_string().as_bytes()[0] as char);

                letter = s[idx];
                cnt = 1;
            }
        }

        ret.push(letter);
        ret.push(cnt.to_string().as_bytes()[0] as char);
    } else {
        let mut idx = 0;

        while idx < s.len() {
            let letter = s[idx];
            let cnt = s[idx + 1].to_digit(10).unwrap();

            for _ in 0..cnt {
                ret.push(letter);
            }

            idx += 2;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
