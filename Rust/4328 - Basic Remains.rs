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

    loop {
        let b = scan.token::<i64>();

        if b == 0 {
            break;
        }

        let (p, m) = (scan.token::<String>(), scan.token::<String>());
        let mut m_converted = 0;
        let mut p_converted = 0;

        for c in m.chars() {
            m_converted = m_converted * b + c.to_digit(10).unwrap() as i64;
        }

        if m_converted == 0 {
            writeln!(out, "{p}").unwrap();
            continue;
        }

        for c in p.chars() {
            p_converted = p_converted * b + c.to_digit(10).unwrap() as i64;
            p_converted %= m_converted;
        }

        if p_converted == 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut stack = Vec::new();

        while p_converted > 0 {
            stack.push(p_converted % b);
            p_converted /= b;
        }

        while let Some(digit) = stack.pop() {
            write!(out, "{digit}").unwrap();
        }

        writeln!(out).unwrap();
    }
}
