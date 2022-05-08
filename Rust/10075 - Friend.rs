use io::Write;
use std::{cmp, io, str};

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

// Reference: https://daily-daily2.tistory.com/17
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut confidence = vec![0; n];
    let mut host = vec![0; n];
    let mut protocol = vec![0; n];

    for i in 0..n {
        confidence[i] = scan.token::<i64>();
    }

    for i in 1..n {
        host[i] = scan.token::<usize>();
        protocol[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    for i in (0..n).rev() {
        match protocol[i] {
            0 => {
                ret += confidence[i];                
                confidence[host[i]] = cmp::max(0, confidence[host[i]] - confidence[i]);
            }
            1 => {
                confidence[host[i]] += confidence[i];
            }
            2 => {
                confidence[host[i]] = cmp::max(confidence[host[i]], confidence[i]);
            }
            _ => panic!("Invalid protocol"),
        }
    }

    writeln!(out, "{ret}").unwrap();
}
