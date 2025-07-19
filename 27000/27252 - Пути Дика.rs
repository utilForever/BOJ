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

    let s = scan.token::<String>();
    let mut height = 0;
    let mut height_max = 0;
    let mut heights = vec![0; s.len()];

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            heights[i] = 1;
            height += 1;
        } else {
            heights[i] = -1;
            height -= 1;
        }

        height_max = height_max.max(height);
    }

    let height_max = height_max as usize;
    let mut height = 0;
    let mut ret = vec![vec!['.'; s.len()]; height_max];

    for i in 0..s.len() {
        if heights[i] == 1 {
            height += heights[i];
            ret[height_max - height as usize][i] = '/';
        } else {
            ret[height_max - height as usize][i] = '\\';
            height += heights[i];
        }
    }

    for i in 0..height_max {
        for j in 0..s.len() {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
