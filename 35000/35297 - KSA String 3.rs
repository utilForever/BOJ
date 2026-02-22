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

    let x = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = x.len();

    let mut idx = n;
    let mut ret = 0;

    for i in (1..=n).rev() {
        let ch = match i % 3 {
            1 => 'K',
            2 => 'S',
            0 => 'A',
            _ => unreachable!(),
        };

        while idx > 0 {
            if (idx & 1) == (i & 1) && x[idx - 1] == ch {
                break;
            }

            idx -= 1;
        }

        if idx == 0 {
            break;
        }

        idx -= 1;
        ret += 1;
    }

    writeln!(out, "{}", 2 * (n - ret)).unwrap();
}
