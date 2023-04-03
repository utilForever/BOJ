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

    let (c, h) = (scan.token::<i32>(), scan.token::<i32>());
    let mut times = [0; 60 * 60 * 24];

    for _ in 0..(c + h) {
        let time = scan.token::<String>();
        let (h, m, s) = (
            time[0..2].parse::<i64>().unwrap(),
            time[3..5].parse::<i64>().unwrap(),
            time[6..8].parse::<i64>().unwrap(),
        );
        let time = (h * 60 * 60 + m * 60 + s) as usize;

        times[time..time + 40].iter_mut().for_each(|x| *x += 1);
    }

    writeln!(out, "{}", times.iter().filter(|&&x| x == 0).count()).unwrap();
}
