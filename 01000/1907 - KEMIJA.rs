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

fn parse_molecule(molecule: &str) -> [i64; 3] {
    let mut ret = [0; 3];
    let mut molecule = molecule.chars().peekable();

    while let Some(c) = molecule.next() {
        let idx = match c {
            'C' => 0,
            'H' => 1,
            'O' => 2,
            _ => unreachable!(),
        };
        let cnt = molecule
            .peek()
            .and_then(|c| c.to_digit(10))
            .map(|d| {
                molecule.next();
                d as i64
            })
            .unwrap_or(1);

        ret[idx] += cnt;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let equation = scan.token::<String>();
    let equation_parts = equation.split(|c| c == '+' || c == '=').collect::<Vec<_>>();
    let (m1, m2, m3) = (
        parse_molecule(equation_parts[0]),
        parse_molecule(equation_parts[1]),
        parse_molecule(equation_parts[2]),
    );

    for x in 1..=10 {
        for y in 1..=10 {
            for z in 1..=10 {
                let left = [
                    m1[0] * x + m2[0] * y,
                    m1[1] * x + m2[1] * y,
                    m1[2] * x + m2[2] * y,
                ];
                let right = [m3[0] * z, m3[1] * z, m3[2] * z];

                if left == right {
                    writeln!(out, "{x} {y} {z}").unwrap();
                    return;
                }
            }
        }
    }

    writeln!(out, "NEMOGUCE").unwrap();
}
