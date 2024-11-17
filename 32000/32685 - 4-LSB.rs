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

    let (a, b, c) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut a_binary = String::new();
    let mut b_binary = String::new();
    let mut c_binary = String::new();

    let a_last_4 = a & 15;
    let b_last_4 = b & 15;
    let c_last_4 = c & 15;

    for i in 0..4 {
        a_binary.push_str(&((a_last_4 >> i) & 1).to_string());
        b_binary.push_str(&((b_last_4 >> i) & 1).to_string());
        c_binary.push_str(&((c_last_4 >> i) & 1).to_string());
    }

    a_binary = a_binary.chars().rev().collect();
    b_binary = b_binary.chars().rev().collect();
    c_binary = c_binary.chars().rev().collect();

    let mut ret = String::new();
    ret.push_str(&a_binary);
    ret.push_str(&b_binary);
    ret.push_str(&c_binary);

    let mut ret_decimal = 0;

    for (i, c) in ret.chars().rev().enumerate() {
        if c == '1' {
            ret_decimal += 2i64.pow(i as u32);
        }
    }

    writeln!(out, "{:04}", ret_decimal).unwrap();
}
