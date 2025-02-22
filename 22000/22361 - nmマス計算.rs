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

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        let mut row = vec![0; n];
        let mut col = vec![0; m];

        for i in 0..n {
            row[i] = scan.token::<i64>();
        }

        for i in 0..m {
            col[i] = scan.token::<i64>();
        }

        let mut digits = [0; 10];

        for i in 0..n {
            for j in 0..m {
                let mut val = row[i] * col[j];

                while val > 0 {
                    digits[(val % 10) as usize] += 1;
                    val /= 10;
                }
            }
        }

        for cnt in digits {
            write!(out, "{cnt} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
