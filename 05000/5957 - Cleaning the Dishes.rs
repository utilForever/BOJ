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

    let mut unwashed = Vec::new();
    let mut washed_but_not_dried = Vec::new();
    let mut washed_and_dried = Vec::new();

    let n = scan.token::<i64>();

    for i in (1..=n).rev() {
        unwashed.push(i);
    }

    loop {
        let (c, d) = (scan.token::<i64>(), scan.token::<i64>());

        if c == 1 {
            for _ in 0..d {
                if let Some(x) = unwashed.pop() {
                    washed_but_not_dried.push(x);
                } else {
                    break;
                }
            }
        } else {
            for _ in 0..d {
                if let Some(x) = washed_but_not_dried.pop() {
                    washed_and_dried.push(x);
                } else {
                    break;
                }
            }
        }

        if unwashed.is_empty() && washed_but_not_dried.is_empty() {
            break;
        }
    }

    while let Some(dish) = washed_and_dried.pop() {
        writeln!(out, "{dish}").unwrap();
    }
}
