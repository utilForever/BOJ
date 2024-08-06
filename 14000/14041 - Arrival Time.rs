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
    let time = time.split(':').collect::<Vec<&str>>();
    let (h, m) = (
        time[0].parse::<i64>().unwrap(),
        time[1].parse::<i64>().unwrap(),
    );
    let time = h * 60 + m;

    let (h, m) = if time > 5 * 60 && time < 10 * 60 {
        if time < 6 * 60 + 40 {
            let a = 420 - time;
            let b = 120 - a;
            let ret = 420 + 2 * b;

            (ret / 60, ret % 60)
        } else if time == 6 * 60 + 40 {
            let a = 420 - time + 90;
            let b = 120 - a;
            let ret = 600 + b;

            (ret / 60, ret % 60)
        } else {
            let a = 600 - time;
            let b = 240 - a;
            let ret = 600 + b / 2;

            (ret / 60, ret % 60)
        }
    } else if time > 13 * 60 && time < 19 * 60 {
        if time < 15 * 60 {
            let a = 900 - time;
            let b = 120 - a;
            let ret = 900 + 2 * b;

            (ret / 60, ret % 60)
        } else {
            let a = 1140 - time;
            let b = 240 - a;
            let ret = 1140 + b / 2;

            (ret / 60, ret % 60)
        }
    } else {
        let mut time = time + 120;

        if time >= 24 * 60 {
            time -= 24 * 60;
        }

        (time / 60, time % 60)
    };

    writeln!(out, "{:02}:{:02}", h, m).unwrap();
}
