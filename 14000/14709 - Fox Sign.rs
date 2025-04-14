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

    let n = scan.token::<i64>();
    let mut checks = [false; 3];

    for _ in 0..n {
        let (mut a, mut b) = (scan.token::<usize>(), scan.token::<usize>());

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        if a == 1 && b == 3 {
            checks[0] = true;
        } else if a == 1 && b == 4 {
            checks[1] = true;
        } else if a == 3 && b == 4 {
            checks[2] = true;
        }
    }

    if n != 3 {
        writeln!(out, "Woof-meow-tweet-squeek").unwrap();
        return;
    }

    writeln!(
        out,
        "{}",
        if checks[0] && checks[1] && checks[2] {
            "Wa-pa-pa-pa-pa-pa-pow!"
        } else {
            "Woof-meow-tweet-squeek"
        }
    )
    .unwrap();
}
