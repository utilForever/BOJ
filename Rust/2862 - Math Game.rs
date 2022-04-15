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

    let mut fibonacci = vec![0; 80];
    fibonacci[1] = 1;
    fibonacci[2] = 1;

    for i in 3..80 {
        fibonacci[i] = fibonacci[i - 1] + fibonacci[i - 2];
    }

    let n = scan.token::<usize>();
    let mut n_copy = n;

    loop {
        let mut temp = n_copy;

        for i in 1..80 {
            if temp < fibonacci[i] {
                temp = fibonacci[i - 1];
                break;
            }
        }

        if temp == n_copy {
            break;
        }

        n_copy -= temp;
    }

    writeln!(out, "{}", n_copy).unwrap();
}
