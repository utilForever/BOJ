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

    let n = scan.token::<usize>();
    let mut boxes = vec![(0, 0, 0); n];

    for i in 0..n {
        let mut b = [
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        ];
        b.sort();

        boxes[i] = (b[0], b[1], b[2]);
    }

    boxes.sort_by(|a, b| (a.0 * a.1 * a.2).cmp(&(b.0 * b.1 * b.2)));

    let m = scan.token::<i64>();

    for _ in 0..m {
        let mut package = [
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        ];
        package.sort();

        if let Some((w, h, l)) = boxes
            .iter()
            .find(|(w, h, l)| w >= &package[0] && h >= &package[1] && l >= &package[2])
        {
            writeln!(out, "{}", w * h * l).unwrap();
        } else {
            writeln!(out, "Item does not fit.").unwrap();
        }
    }
}
