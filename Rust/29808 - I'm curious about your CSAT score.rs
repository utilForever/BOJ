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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = scan.token::<i64>();

    if s % 4763 != 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    s /= 4763;

    let mut ret = BTreeSet::new();

    for a in 0..=200 {
        for b in 0..=200 {
            let candidates1 = [a * 508, a * 108];
            let candidates2 = [b * 212, b * 305];

            'choose_candidate: for i in 0..2 {
                for j in 0..2 {
                    if candidates1[i] + candidates2[j] == s {
                        ret.insert((a, b));
                        break 'choose_candidate;
                    }
                }
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for (num1, num2) in ret {
        writeln!(out, "{num1} {num2}").unwrap();
    }
}
