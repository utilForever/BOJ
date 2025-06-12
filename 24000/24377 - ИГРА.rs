use io::Write;
use std::{io, str, vec};

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

    let mut boxes = [0; 4];

    for i in 0..4 {
        boxes[i] = scan.token::<i32>();
    }

    let cnt_zero = boxes.iter().filter(|&&x| x == 0).count();

    if cnt_zero == 0 {
        writeln!(out, "{} {}", boxes[0], boxes[1]).unwrap();
    } else if cnt_zero == 1 {
        let mut check = [false; 4];
        let mut pos_zero = 0;

        for i in 0..4 {
            if boxes[i] == 0 {
                pos_zero = i + 1;
            } else {
                check[boxes[i] as usize - 1] = true;
            }
        }

        let num = check.iter().position(|&x| !x).unwrap() as i32 + 1;

        writeln!(out, "{pos_zero} {num}").unwrap();
    } else {
        let mut check = [false; 4];

        for i in 0..4 {
            if boxes[i] != 0 {
                check[boxes[i] as usize - 1] = true;
            }
        }

        for i in 0..4 {
            if !check[i] {
                write!(out, "{} ", i + 1).unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
