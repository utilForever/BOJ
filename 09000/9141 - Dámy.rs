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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<i64>();

        if n == 0 {
            break;
        }

        writeln!(
            out,
            "{}",
            if gcd(n, 6) != 1 {
                "Kralovny se nevejdou."
            } else {
                "Kralovny lze umistit."
            }
        )
        .unwrap();
    }
}
