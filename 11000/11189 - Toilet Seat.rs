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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut policy1 = s[0];
    let mut policy2 = s[0];
    let mut policy3 = s[0];
    let mut cnt_policy1 = 0;
    let mut cnt_policy2 = 0;
    let mut cnt_policy3 = 0;

    for &c in s.iter().skip(1) {
        // Policy 1
        if policy1 != c {
            cnt_policy1 += 1;
            policy1 = c;
        }

        if policy1 != 'U' {
            cnt_policy1 += 1;
            policy1 = 'U';
        }

        // Policy 2
        if policy2 != c {
            cnt_policy2 += 1;
            policy2 = c;
        }

        if policy2 != 'D' {
            cnt_policy2 += 1;
            policy2 = 'D';
        }

        // Policy 3
        if policy3 != c {
            cnt_policy3 += 1;
            policy3 = c;
        }
    }

    writeln!(out, "{cnt_policy1}").unwrap();
    writeln!(out, "{cnt_policy2}").unwrap();
    writeln!(out, "{cnt_policy3}").unwrap();
}
