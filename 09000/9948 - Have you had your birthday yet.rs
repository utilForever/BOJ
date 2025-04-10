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

    loop {
        let (day, month) = (scan.token::<i64>(), scan.token::<String>());

        if day == 0 && month == "#" {
            break;
        }

        if month == "February" && day == 29 {
            writeln!(out, "Unlucky").unwrap();
            continue;
        }

        if month == "August" {
            writeln!(
                out,
                "{}",
                if day == 4 {
                    "Happy birthday"
                } else if day < 4 {
                    "Yes"
                } else {
                    "No"
                }
            )
            .unwrap();
            continue;
        }

        writeln!(
            out,
            "{}",
            if month == "September"
                || month == "October"
                || month == "November"
                || month == "December"
            {
                "No"
            } else {
                "Yes"
            }
        )
        .unwrap();
    }
}
