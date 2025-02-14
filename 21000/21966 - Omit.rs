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

    let n = scan.token::<usize>();
    let s = scan.token::<String>().chars().collect::<Vec<_>>();

    if n <= 25 {
        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
        return;
    }

    let left = 11;
    let right = n - 11;
    let mut check = true;

    for i in left..right - 1 {
        if s[i] == '.' {
            check = false;
            break;
        }
    }

    if check {
        writeln!(
            out,
            "{}",
            s.iter().take(11).collect::<String>()
                + "..."
                + &s.iter().skip(n - 11).collect::<String>()
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "{}",
            s.iter().take(9).collect::<String>()
                + "......"
                + &s.iter().skip(n - 10).collect::<String>()
        )
        .unwrap();
    }
}
