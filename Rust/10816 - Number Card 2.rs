use io::Write;
use std::{collections::HashMap, io, str};

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

    let n = scan.token::<usize>();
    let mut cards = HashMap::new();

    for _ in 0..n {
        let num = scan.token::<i64>();

        if cards.contains_key(&num) {
            *cards.get_mut(&num).unwrap() += 1;
        } else {
            cards.insert(num, 1);
        }
    }

    let m = scan.token::<usize>();

    for _ in 0..m {
        let num = scan.token::<i64>();

        write!(
            out,
            "{} ",
            if cards.contains_key(&num) {
                *cards.get(&num).unwrap()
            } else {
                0
            }
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}
