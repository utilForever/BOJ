use io::Write;
use std::{collections::BTreeSet, io, str};

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
    let mut boys_odd = BTreeSet::new();
    let mut boys_even = BTreeSet::new();

    for _ in 0..n {
        let boy = scan.token::<i64>();

        if boy % 2 == 0 {
            boys_even.insert(boy);
        } else {
            boys_odd.insert(boy);
        }
    }

    if boys_even.len() > boys_odd.len() {
        let mut ret = 2;

        loop {
            if boys_even.contains(&ret) {
                ret += 2;
            } else {
                writeln!(out, "{ret}").unwrap();
                break;
            }
        }
    } else {
        let mut ret = 1;

        loop {
            if boys_odd.contains(&ret) {
                ret += 2;
            } else {
                writeln!(out, "{ret}").unwrap();
                break;
            }
        }
    }
}
