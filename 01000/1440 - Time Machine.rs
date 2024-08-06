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
    let time = time.split(":").collect::<Vec<_>>();

    let time1 = time[0].parse::<i64>().unwrap();
    let time2 = time[1].parse::<i64>().unwrap();
    let time3 = time[2].parse::<i64>().unwrap();
    let times = vec![time1, time2, time3];
    let mut ret = 0;

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                if i == j || j == k || k == i {
                    continue;
                }

                if times[i] >= 1
                    && times[i] <= 12
                    && times[j] >= 0
                    && times[j] <= 59
                    && times[k] >= 0
                    && times[k] <= 59
                {
                    ret += 1;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
