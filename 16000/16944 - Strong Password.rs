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

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut conditions = [false; 4];
    let length = s.len();

    for c in s {
        if c.is_uppercase() {
            conditions[0] = true;
        }

        if c.is_lowercase() {
            conditions[1] = true;
        }

        if c.is_numeric() {
            conditions[2] = true;
        }

        if c == '!'
            || c == '@'
            || c == '#'
            || c == '$'
            || c == '%'
            || c == '^'
            || c == '&'
            || c == '*'
            || c == '('
            || c == ')'
            || c == '-'
            || c == '+'
        {
            conditions[3] = true;
        }
    }

    let false_conditions = conditions.iter().filter(|&&x| x == false).count();

    writeln!(
        out,
        "{}",
        6_usize.saturating_sub(length).max(false_conditions)
    )
    .unwrap();
}
