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

    let n = scan.token::<i64>();
    let levels_arcane_river = [
        [200, 210, 220],
        [210, 220, 225],
        [220, 225, 230],
        [225, 230, 235],
        [230, 235, 245],
        [235, 245, 250],
    ];
    let levels_grandis = [
        [260, 265, 270],
        [265, 270, 275],
        [270, 275, 280],
        [275, 280, 285],
        [280, 285, 290],
        [285, 290, 295],
        [290, 295, 300],
    ];

    for i in 0..6 {
        write!(
            out,
            "{} ",
            if n < levels_arcane_river[i][0] {
                0
            } else if n < levels_arcane_river[i][1] {
                500
            } else if n < levels_arcane_river[i][2] {
                300
            } else {
                100
            }
        )
        .unwrap();
    }

    writeln!(out).unwrap();

    for i in 0..7 {
        write!(
            out,
            "{} ",
            if n < levels_grandis[i][0] {
                0
            } else if n < levels_grandis[i][1] {
                500
            } else if n < levels_grandis[i][2] {
                300
            } else {
                100
            }
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}
