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

    let (w, h) = (scan.token::<usize>(), scan.token::<usize>());
    let mut first = vec![vec![0; w]; h];
    let mut second = vec![vec![0; w]; h];

    for i in 0..h {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            first[i][j] = c as u8 - b'0';
        }
    }

    for i in 0..h {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            second[i][j] = c as u8 - b'0';
        }
    }

    let table = scan
        .token::<String>()
        .chars()
        .map(|c| c as u8 - b'0')
        .collect::<Vec<_>>();

    for i in 0..h {
        for j in 0..w {
            write!(
                out,
                "{}",
                match (first[i][j], second[i][j]) {
                    (0, 0) => table[0],
                    (0, 1) => table[1],
                    (1, 0) => table[2],
                    (1, 1) => table[3],
                    _ => unreachable!(),
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
