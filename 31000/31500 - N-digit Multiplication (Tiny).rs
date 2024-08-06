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

    let n = scan.token::<i64>();
    let a = scan.token::<String>();
    let b = scan.token::<String>();
    if a == "!" || b == "!"{
        write!(out, "!").unwrap();
        return;
    }
    let mut a_converted = Vec::new();
    let mut b_converted = Vec::new();

    for c in a.chars().rev() {
        a_converted.push(c as i64 - '!' as i64);
    }

    for c in b.chars().rev() {
        b_converted.push(c as i64 - '!' as i64);
    }

    let mut is_minus = false;

    if n > 0 && a.chars().next().unwrap() == '~' {
        is_minus ^= true;
        a_converted.pop();
    }

    if n > 0 && b.chars().next().unwrap() == '~' {
        is_minus ^= true;
        b_converted.pop();
    }

    let mut ret = vec![0; a_converted.len() + b_converted.len()];

    for i in 0..a_converted.len() {
        for j in 0..b_converted.len() {
            ret[i + j] += a_converted[i] * b_converted[j];
        }
    }

    if n < 0 {
        ret.push(0);
    }

    for i in 0..ret.len() - 1 {
        ret[i + 1] += ret[i] / n;
        ret[i] %= n;

        if n < 0 && ret[i] < 0 {
            ret[i] -= n;
            ret[i + 1] += 1;
        }
    }

    while !ret.is_empty() && ret.last() == Some(&0) {
        ret.pop();
    }

    if ret.is_empty() {
        ret.push(0);
    }

    ret.reverse();

    if is_minus {
        write!(out, "~").unwrap();
    }

    for i in 0..ret.len() {
        write!(out, "{}", (ret[i] + '!' as i64) as u8 as char).unwrap();
    }
}
