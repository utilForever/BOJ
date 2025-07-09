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

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut stars = vec![(0.0, 0.0, 0.0); n];

        for i in 0..n {
            let (x, y, z) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            let len = (x * x + y * y + z * z).sqrt();
            stars[i] = (x / len, y / len, z / len);
        }

        let m = scan.token::<usize>();
        let mut telescopes = vec![(0.0, 0.0, 0.0, 0.0); m];

        for i in 0..m {
            let (x, y, z, psi) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            let len = (x * x + y * y + z * z).sqrt();
            telescopes[i] = (x / len, y / len, z / len, psi.cos());
        }

        let mut ret = 0;

        'outer: for i in 0..n {
            for j in 0..m {
                let dot = stars[i].0 * telescopes[j].0
                    + stars[i].1 * telescopes[j].1
                    + stars[i].2 * telescopes[j].2;

                if dot - telescopes[j].3 > 1e-9 {
                    ret += 1;
                    continue 'outer;
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
