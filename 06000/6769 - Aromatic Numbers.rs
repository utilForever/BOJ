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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut idx = 0;
    let mut ret = 0;

    let convert_roman_num = |roman: char| -> i64 {
        match roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        }
    };

    while idx < s.len() {
        if s[idx].is_numeric() {
            let num = s[idx].to_digit(10).unwrap() as i64;
            idx += 1;

            let roman = s[idx];
            idx += 1;

            let roman_num_curr = convert_roman_num(roman);
            let roman_num_next = if idx + 1 < s.len() {
                convert_roman_num(s[idx + 1])
            } else {
                0
            };

            ret += if roman_num_curr < roman_num_next {
                -num * roman_num_curr
            } else {
                num * roman_num_curr
            };
        }
    }

    writeln!(out, "{ret}").unwrap();
}
