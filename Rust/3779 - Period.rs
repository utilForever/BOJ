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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut idx = 1;

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        if idx > 1 {
            writeln!(out).unwrap();
        }

        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();

        let mut cmp = 0;
        let mut fail = vec![0; n];

        for i in 1..n {
            while cmp > 0 && s[cmp] != s[i] {
                cmp = fail[cmp - 1] as usize;
            }

            if s[cmp] == s[i] {
                cmp += 1;
                fail[i] = cmp as i64;
            }
        }

        writeln!(out, "Test case #{idx}").unwrap();

        for i in 0..n {
            let len = i as i64 + 1 - fail[i];

            if fail[i] > 0 && (i as i64 + 1) % len == 0 {
                writeln!(out, "{} {}", i + 1, (i as i64 + 1) / len).unwrap();
            }
        }

        idx += 1;
    }
}
