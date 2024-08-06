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

    let time = scan.token::<String>();
    let (hh, mm) = (
        time[0..2].parse::<i64>().unwrap(),
        time[3..5].parse::<i64>().unwrap(),
    );
    let mut time = hh * 60 + mm;
    let n = scan.token::<i64>();

    if time % 15 != 0 {
        time += 15 - time % 15;
    }

    let mut cnt = 0;

    loop {
        if time % 60 == 0 {
            cnt += (time / 60 - 1) % 12 + 1;
        } else {
            cnt += 1;
        }

        if cnt >= n {
            break;
        }

        time += 15;
    }

    let (hh, mm) = ((time / 60 - 1) % 12 + 1, time % 60);

    writeln!(out, "{:02}:{:02}", hh, mm).unwrap();
}
