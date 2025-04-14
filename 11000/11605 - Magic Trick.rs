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
    let mut operations = vec![(String::new(), 0); n];

    for i in 0..n {
        operations[i] = (scan.token::<String>(), scan.token::<i64>());
    }

    let mut ret = 0;

    for i in 1..=100 {
        let mut val = i;
        let mut check = true;

        for op in operations.iter() {
            match op.0.as_str() {
                "ADD" => {
                    val += op.1;
                }
                "SUBTRACT" => {
                    val -= op.1;

                    if val < 0 {
                        check = false;
                        break;
                    }
                }
                "MULTIPLY" => {
                    val *= op.1;
                }
                "DIVIDE" => {
                    if val % op.1 != 0 {
                        check = false;
                        break;
                    }

                    val /= op.1;
                }
                _ => unreachable!(),
            }
        }

        if !check {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
