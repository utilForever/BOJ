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

    let p = scan.token::<i64>();
    let mut cnt_software = 0;
    let mut cnt_embedded = 0;
    let mut cnt_ai = 0;
    let mut cnt_etc = 0;

    for _ in 0..p {
        let (g, c, _) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if g == 2 || g == 3 {
            match c {
                1 | 2 => cnt_software += 1,
                3 => cnt_embedded += 1,
                4 => cnt_ai += 1,
                _ => unreachable!(),
            }
        } else {
            cnt_etc += 1;
        }
    }

    writeln!(out, "{cnt_software}").unwrap();
    writeln!(out, "{cnt_embedded}").unwrap();
    writeln!(out, "{cnt_ai}").unwrap();
    writeln!(out, "{cnt_etc}").unwrap();
}
