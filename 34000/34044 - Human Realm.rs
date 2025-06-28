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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    if n == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    if n % 2 == 0 {
        let len = n / 2;

        for _ in 0..len {
            write!(out, "2937").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        let mut cnt = 0;

        for _ in 0..n - 2 {
            write!(out, "43").unwrap();
            cnt = (cnt + 1) % 11;
        }

        writeln!(
            out,
            "{}",
            match cnt {
                0 => "4323",
                1 => "4313",
                2 => "4347",
                3 => "4337",
                4 => "4371",
                5 => "4317",
                6 => "4329",
                7 => "4319",
                8 => "4331",
                9 => "4343",
                10 => "3123",
                _ => unreachable!(),
            }
        )
        .unwrap();
    }
}
