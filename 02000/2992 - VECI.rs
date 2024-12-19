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

    let x = scan.token::<i64>();

    if x < 10 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut digits = Vec::new();
    let mut num = x;

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    let mut check = true;

    digits.windows(2).for_each(|window| {
        if window[0] > window[1] {
            check = false;
        }
    });

    if check {
        writeln!(out, "0").unwrap();
        return;
    }

    digits.sort();

    for val in x + 1..=999999 {
        let mut digits_val = Vec::new();
        let mut num_val = val;

        while num_val > 0 {
            digits_val.push(num_val % 10);
            num_val /= 10;
        }

        digits_val.sort();

        if digits == digits_val {
            writeln!(out, "{val}").unwrap();
            return;
        }
    }

    writeln!(out, "0").unwrap();
}
