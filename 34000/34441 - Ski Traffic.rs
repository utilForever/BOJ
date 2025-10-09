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

    let _ = scan.line().trim().to_string();
    let time = scan.token::<String>();
    let time = time.split(':').collect::<Vec<&str>>();
    let mut time = time[0].parse::<i64>().unwrap() * 60 + time[1].parse::<i64>().unwrap();

    let day = scan.token::<String>();

    if day == "sat" || day == "sun" {
        time *= 2;
    }

    let weather = scan.token::<i64>();

    if weather == 1 {
        time *= 2;
    }

    let snowed = scan.token::<i64>();

    if snowed == 1 {
        time *= 3;
    }

    let holiday = scan.token::<i64>();

    if holiday == 1 {
        time *= 3;
    }

    writeln!(out, "{}:{:02}", time / 60, time % 60).unwrap();
}
