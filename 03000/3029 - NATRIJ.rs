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

    let current_time = scan.token::<String>();
    let current_time = current_time
        .split(':')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let throw_time = scan.token::<String>();
    let throw_time = throw_time
        .split(':')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut second = throw_time[2] - current_time[2];
    let mut minute = throw_time[1] - current_time[1];
    let mut hour = throw_time[0] - current_time[0];

    if second < 0 {
        second += 60;
        minute -= 1;
    }

    if minute < 0 {
        minute += 60;
        hour -= 1;
    }

    if hour < 0 {
        hour += 24;
    }

    if hour == 0 && minute == 0 && second == 0 {
        hour = 24;
    }

    writeln!(out, "{:02}:{:02}:{:02}", hour, minute, second).unwrap();
}
