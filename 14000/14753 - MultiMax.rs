use io::Write;
use std::{i64, io, str};

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
    let mut cards = vec![0; n];

    for i in 0..n {
        cards[i] = scan.token::<i64>();
    }

    cards.sort_unstable();

    let a = cards[0];
    let b = cards[1];
    let c = cards[n - 3];
    let d = cards[n - 2];
    let e = cards[n - 1];

    let candidate1 = c * d * e;
    let candidate2 = a * b * e;
    let candidate3 = d * e;
    let candidate4 = a * b;

    writeln!(
        out,
        "{}",
        [candidate1, candidate2, candidate3, candidate4]
            .iter()
            .max()
            .unwrap()
    )
    .unwrap();
}
