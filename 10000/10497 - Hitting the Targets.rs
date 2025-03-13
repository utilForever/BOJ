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

enum Shape {
    Rectangle(i64, i64, i64, i64),
    Circle(i64, i64, i64),
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let m = scan.token::<usize>();
    let mut shapes = Vec::with_capacity(m);

    for _ in 0..m {
        let shape = scan.token::<String>();

        if shape == "rectangle" {
            let (x1, y1, x2, y2) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
            shapes.push(Shape::Rectangle(x1, y1, x2, y2));
        } else {
            let (x, y, r) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
            shapes.push(Shape::Circle(x, y, r));
        }
    }

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let mut ret = 0;

        for shape in shapes.iter() {
            match shape {
                Shape::Rectangle(x1, y1, x2, y2) => {
                    if x >= *x1 && x <= *x2 && y >= *y1 && y <= *y2 {
                        ret += 1;
                    }
                }
                Shape::Circle(cx, cy, r) => {
                    if (x - cx).pow(2) + (y - cy).pow(2) <= r.pow(2) {
                        ret += 1;
                    }
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
