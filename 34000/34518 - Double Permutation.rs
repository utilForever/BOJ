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

    let n = scan.token::<usize>();

    if n % 4 == 2 || n % 4 == 3 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut ret_a = vec![1; n + 1];
    let mut ret_b = vec![1; n + 1];

    for i in (0..n / 2).step_by(2) {
        ret_a[i] = n - 2 * i;   
        ret_b[n - 1 - i] = n - 2 * i;

        ret_b[i + 1] = n - 2 * i - 1;
        ret_a[n - 1 - i] = n - 2 * i - 1;

        ret_b[i] = n - 2 * i - 2;
        ret_b[n - 2 - i] = n - 2 * i - 2;

        ret_a[i + 1] = n - 2 * i - 3;
        ret_a[n - 2 - i] = n - 2 * i - 3;
    }

    for i in 0..n {
        write!(out, "{} ", ret_a[i]).unwrap();
    }

    writeln!(out).unwrap();

    for i in 0..n {
        write!(out, "{} ", ret_b[i]).unwrap();
    }

    writeln!(out).unwrap();
}
