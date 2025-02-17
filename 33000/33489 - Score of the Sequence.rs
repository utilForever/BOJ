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

    let mut fibonacci = Vec::new();

    fibonacci.push(0);
    fibonacci.push(1);
    fibonacci.push(1);

    let mut i = 3;

    loop {
        let next = fibonacci[i - 1] + fibonacci[i - 2];

        if next > 300_000 {
            break;
        }

        fibonacci.push(next);
        i += 1;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        let mut ret = 1;

        for i in 1..fibonacci.len() {
            if i + 1 < fibonacci.len() && fibonacci[i + 1] <= x && fibonacci[i] <= y {
                ret = i;
            } else {
                break;
            }
        }

        writeln!(out, "{} {}", fibonacci[ret + 1], fibonacci[ret]).unwrap();
    }
}
