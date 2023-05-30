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

    let _ = scan.token::<i64>();
    let directions = scan.token::<String>();
    let directions = directions.chars().collect::<Vec<_>>();

    let k = scan.token::<usize>();
    let mut coordinates = vec![(0, 0); k];

    for i in 0..k {
        coordinates[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut cnt_r = 0;
    let mut cnt_u = 0;
    let mut cnt_x = 0;

    for direction in directions {
        match direction {
            'R' => cnt_r += 1,
            'U' => cnt_u += 1,
            'X' => cnt_x += 1,
            _ => unreachable!(),
        }
    }

    let mut ret = 0;

    for coordinate in coordinates {
        if cnt_r + cnt_x >= coordinate.0 - 1
            && cnt_u + cnt_x >= coordinate.1 - 1
            && coordinate.0 - coordinate.1 <= cnt_r
            && coordinate.1 - coordinate.0 <= cnt_u
        {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
