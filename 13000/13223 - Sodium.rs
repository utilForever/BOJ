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

    let time_a = scan.token::<String>();
    let time_b = scan.token::<String>();

    let time_a = time_a.split(":").collect::<Vec<&str>>();
    let time_b = time_b.split(":").collect::<Vec<&str>>();

    let time_a = time_a
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let time_b = time_b
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut ret_hour = time_b[0] - time_a[0];
    let mut ret_min = time_b[1] - time_a[1];
    let mut ret_sec = time_b[2] - time_a[2];

    if ret_sec < 0 {
        ret_sec += 60;
        ret_min -= 1;
    }

    if ret_min < 0 {
        ret_min += 60;
        ret_hour -= 1;
    }

    if ret_hour < 0 {
        ret_hour += 24;
    }

    if ret_hour == 0 && ret_min == 0 && ret_sec == 0 {
        ret_hour = 24;
    }

    writeln!(out, "{:02}:{:02}:{:02}", ret_hour, ret_min, ret_sec).unwrap();
}
