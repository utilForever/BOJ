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

    let time_alarm = scan.token::<String>();
    let time_look_at = scan.token::<String>();
    let time_alarm = {
        let parts = time_alarm
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        parts[0] * 60 + parts[1]
    };
    let time_look_at = {
        let parts = time_look_at
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        parts[0] * 60 + parts[1]
    };

    writeln!(
        out,
        "{}",
        if time_alarm <= time_look_at {
            "YES"
        } else {
            "NO"
        }
    )
    .unwrap();
}
