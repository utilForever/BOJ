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

    let mut nums = [0; 10];

    for i in 0..=9 {
        let idx = scan.token::<usize>();
        nums[idx] = i;
    }

    let (a, b) = (scan.token::<String>(), scan.token::<String>());
    let mut a_converted = String::new();
    let mut b_converted = String::new();

    for c in a.chars() {
        let c_converted = nums[(c as u8 - b'0') as usize];
        a_converted.push((c_converted as u8 + b'0') as char);
    }

    for c in b.chars() {
        let c_converted = nums[(c as u8 - b'0') as usize];
        b_converted.push((c_converted as u8 + b'0') as char);
    }

    let (a, b) = (
        a_converted.parse::<i64>().unwrap(),
        b_converted.parse::<i64>().unwrap(),
    );
    let sum = (a + b).to_string();
    let mut ret = String::new();

    for c in sum.chars() {
        let pos = nums
            .iter()
            .position(|x| *x == (c as u8 - b'0') as i32)
            .unwrap();

        if ret.is_empty() && pos == 0 {
            continue;
        }

        ret.push((pos as u8 + b'0') as char);
    }

    if ret.is_empty() {
        ret.push('0');
    }

    writeln!(out, "{ret}").unwrap();
}
