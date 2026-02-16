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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut sides = [0; 4];

        for i in 0..4 {
            sides[i] = scan.token::<i64>();
        }

        sides.sort_unstable();

        let mut acute = 0;
        let mut right = 0;
        let mut obtuse = 0;

        for i in 0..4 {
            for j in i + 1..4 {
                for k in j + 1..4 {
                    let a = sides[i];
                    let b = sides[j];
                    let c = sides[k];

                    if a + b <= c {
                        continue;
                    }

                    if a * a + b * b > c * c {
                        acute += 1;
                    } else if a * a + b * b == c * c {
                        right += 1;
                    } else {
                        obtuse += 1;
                    }
                }
            }
        }

        writeln!(out, "{right} {acute} {obtuse}").unwrap();
    }
}
