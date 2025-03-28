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

    let (district, floors) = (scan.token::<String>(), scan.token::<i64>());

    writeln!(
        out,
        "{}",
        if district == "residential" {
            match floors {
                1 => "0",
                2..=5 => "1",
                6..=10 => "2",
                11..=15 => "3",
                16..=20 => "4",
                _ => unreachable!(),
            }
        } else if district == "commercial" {
            match floors {
                1 => "0",
                2..=7 => "1",
                8..=14 => "2",
                15..=20 => "3",
                _ => unreachable!(),
            }
        } else if district == "industrial" {
            match floors {
                1 => "0",
                2..=4 => "1",
                5..=8 => "2",
                9..=12 => "3",
                13..=16 => "4",
                17..=20 => "5",
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    )
    .unwrap();
}
