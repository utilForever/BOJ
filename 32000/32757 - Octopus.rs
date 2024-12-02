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

    let (n, k) = (scan.token::<i64>(), scan.token::<i64>());

    // If n <= k, we can't make "octopus" with n legs => 0
    // If k is even, we can make every vertices to "octopus" => n
    // If k is odd,
    //     n is even, we can make every vertices to "octopus" => n
    //     n is odd,  we can make every vertices to "octopus" except one => n - 1
    // Reference: https://en.wikipedia.org/wiki/Handshaking_lemma
    writeln!(
        out,
        "{}",
        if n <= k {
            0
        } else if k % 2 == 0 {
            n
        } else if n % 2 == 0 {
            n
        } else {
            n - 1
        }
    )
    .unwrap();
}
