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

    let birthday = scan.token::<String>();
    let birthday = birthday.chars().collect::<Vec<_>>();
    let n = scan.token::<i64>();
    let mut ret = (String::new(), 0);

    for _ in 0..n {
        let coding = scan.token::<String>();
        let coding_chars = coding.chars().collect::<Vec<_>>();
        let mut biorhythm = 0;

        let mut year = 0;

        for i in 0..4 {
            year += (birthday[i] as i64 - coding_chars[i] as i64).pow(2);
        }

        biorhythm += year;

        let mut month = 0;

        for i in 4..6 {
            month += (birthday[i] as i64 - coding_chars[i] as i64).pow(2);
        }

        biorhythm *= month;

        let mut day = 0;

        for i in 6..8 {
            day += (birthday[i] as i64 - coding_chars[i] as i64).pow(2);
        }

        biorhythm *= day;

        if biorhythm > ret.1 || (biorhythm == ret.1 && coding < ret.0) {
            ret = (coding, biorhythm);
        }
    }

    writeln!(out, "{}", ret.0).unwrap();
}
