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

    let n = scan.token::<i64>();
    let mut binaries = Vec::new();

    for i in 0..(1 << n) {
        let mut binary = String::new();
        let mut num = i;

        while num > 0 {
            binary.push_str(&(num % 2).to_string());
            num /= 2;
        }

        while binary.len() < n as usize {
            binary.push('0');
        }

        binaries.push(binary.chars().rev().collect::<String>());
    }

    binaries.sort_by(|a, b| {
        let cnt_a = a.chars().filter(|&c| c == '1').count();
        let cnt_b = b.chars().filter(|&c| c == '1').count();

        if cnt_a == cnt_b {
            let a_rev = a.chars().rev().collect::<String>();
            let b_rev = b.chars().rev().collect::<String>();

            a_rev.cmp(&b_rev)
        } else {
            cnt_a.cmp(&cnt_b)
        }
    });

    let s = scan.token::<String>();

    writeln!(out, "{}", binaries.iter().position(|x| x == &s).unwrap()).unwrap();
}