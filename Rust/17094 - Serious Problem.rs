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
    let mut cnt_two = 0;
    let mut cnt_e = 0;

    for c in s.iter() {
        if *c == '2' {
            cnt_two += 1;
        } else if *c == 'e' {
            cnt_e += 1;
        }
    }

    writeln!(
        out,
        "{}",
        match cnt_two.cmp(&cnt_e) {
            std::cmp::Ordering::Less => "e",
            std::cmp::Ordering::Equal => "yee",
            std::cmp::Ordering::Greater => "2",
        }
    )
    .unwrap();
}
