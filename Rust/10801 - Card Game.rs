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

    let mut card_a = [0; 10];
    let mut card_b = [0; 10];
    let mut win_a = 0;
    let mut win_b = 0;

    for i in 0..10 {
        card_a[i] = scan.token::<i64>();
    }

    for i in 0..10 {
        card_b[i] = scan.token::<i64>();
    }

    for i in 0..10 {
        if card_a[i] > card_b[i] {
            win_a += 1;
        } else if card_a[i] < card_b[i] {
            win_b += 1;
        }
    }

    writeln!(
        out,
        "{}",
        if win_a > win_b {
            "A"
        } else if win_a < win_b {
            "B"
        } else {
            "D"
        }
    )
    .unwrap();
}
