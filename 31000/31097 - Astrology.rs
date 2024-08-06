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

    let date = scan.token::<String>();
    let month = date[5..7].parse::<i64>().unwrap();
    let day = date[8..10].parse::<i64>().unwrap();

    writeln!(
        out,
        "{}",
        if (month == 3 && day >= 21) || (month == 4 && day <= 19) {
            "Aries"
        } else if (month == 4 && day >= 20) || (month == 5 && day <= 20) {
            "Taurus"
        } else if (month == 5 && day >= 21) || (month == 6 && day <= 20) {
            "Gemini"
        } else if (month == 6 && day >= 21) || (month == 7 && day <= 22) {
            "Cancer"
        } else if (month == 7 && day >= 23) || (month == 8 && day <= 22) {
            "Leo"
        } else if (month == 8 && day >= 23) || (month == 9 && day <= 22) {
            "Virgo"
        } else if (month == 9 && day >= 23) || (month == 10 && day <= 22) {
            "Libra"
        } else if (month == 10 && day >= 23) || (month == 11 && day <= 22) {
            "Scorpio"
        } else if (month == 11 && day >= 23) || (month == 12 && day <= 21) {
            "Sagittarius"
        } else if (month == 12 && day >= 22) || (month == 1 && day <= 19) {
            "Capricorn"
        } else if (month == 1 && day >= 20) || (month == 2 && day <= 18) {
            "Aquarius"
        } else if (month == 2 && day >= 19) || (month == 3 && day <= 20) {
            "Pisces"
        } else {
            unreachable!()
        }
    )
    .unwrap();
}
