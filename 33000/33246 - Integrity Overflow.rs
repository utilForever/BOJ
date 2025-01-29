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

    let n = scan.token::<usize>();
    let password = scan.token::<String>();
    let mut ret = true;

    for _ in 0..n {
        let (p, s) = (scan.token::<String>(), scan.token::<String>());

        if password.len() != p.len() {
            if s == "ALLOWED" {
                ret = false;
            }
            
            continue;
        }

        let mut diff = 0;

        for (a, b) in password.chars().zip(p.chars()) {
            if a != b {
                diff += 1;
            }
        }

        if (diff <= 1 && s == "ALLOWED") || (diff > 1 && s == "DENIED") {
            continue;
        }

        ret = false;
    }

    writeln!(
        out,
        "{}",
        if ret {
            "SYSTEM SECURE"
        } else {
            "INTEGRITY OVERFLOW"
        }
    )
    .unwrap();
}
