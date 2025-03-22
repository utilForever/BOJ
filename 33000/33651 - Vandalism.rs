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

    let s = scan.token::<String>();
    let mut uapc = [false, false, false, false];

    for c in s.chars() {
        match c {
            'U' => uapc[0] = true,
            'A' => uapc[1] = true,
            'P' => uapc[2] = true,
            'C' => uapc[3] = true,
            _ => (),
        }
    }

    for i in 0..4 {
        if uapc[i] {
            continue;
        }

        write!(
            out,
            "{}",
            match i {
                0 => "U",
                1 => "A",
                2 => "P",
                3 => "C",
                _ => unreachable!(),
            }
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}
