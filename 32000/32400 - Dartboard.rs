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

    let mut scores = [0; 20];

    for i in 0..20 {
        scores[i] = scan.token::<i64>();
    }

    let pos_20 = scores.iter().position(|&x| x == 20).unwrap();
    let score_alice = (if pos_20 == 0 {
        scores[1] + scores[19] + 20
    } else if pos_20 == 19 {
        scores[0] + scores[18] + 20
    } else {
        scores[pos_20 - 1] + scores[pos_20 + 1] + 20
    }) * 20;
    let score_bob = scores.iter().sum::<i64>() * 3;

    writeln!(
        out,
        "{}",
        if score_alice > score_bob {
            "Alice"
        } else if score_alice < score_bob {
            "Bob"
        } else {
            "Tie"
        }
    )
    .unwrap();
}
