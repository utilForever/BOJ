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

    let n = scan.token::<i64>();
    let mut time_first = i64::MAX;
    let mut time_last = i64::MIN;

    for _ in 0..n {
        let time = scan.token::<String>();

        let time = time
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let time = time[0] * 60 + time[1];

        if time < 390 || time > 1140 {
            continue;
        }

        time_first = time_first.min(time);
        time_last = time_last.max(time);
    }

    writeln!(
        out,
        "{}",
        if time_first >= 390 && time_first <= 600 {
            if time_last >= 390 && time_last <= 960 {
                24000
            } else if time_last >= 961 && time_last <= 1140 {
                36000
            } else {
                0
            }
        } else if time_first >= 601 && time_first <= 960 && time_last >= 601 && time_last <= 960 {
            16800
        } else if time_first <= 601 && time_first <= 1140 && time_last >= 961 && time_last <= 1140 {
            24000
        } else {
            0
        }
    )
    .unwrap();
}
