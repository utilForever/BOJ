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

    let (r, g, b) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let (r_dash, g_dash, b_dash) = (r / 255.0, g / 255.0, b / 255.0);
    let k = 1.0 - r_dash.max(g_dash).max(b_dash);

    if k == 1.0 {
        writeln!(out, "0.0 0.0 0.0 1.0").unwrap();
        return;
    }

    let c = (1.0 - r_dash - k) / (1.0 - k);
    let m = (1.0 - g_dash - k) / (1.0 - k);
    let y = (1.0 - b_dash - k) / (1.0 - k);

    writeln!(out, "{:.9} {:.9} {:.9} {:.9}", c, m, y, k).unwrap();
}
