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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let _ = scan.token::<i64>();
    let mut dna = scan.token::<String>().chars().collect::<Vec<_>>();

    while dna.len() > 1 {
        let b = dna.pop().unwrap();
        let a = dna.pop().unwrap();

        match (a, b) {
            ('A', 'A') | ('A', 'C') | ('G', 'T') | ('C', 'A') | ('T', 'G') => {
                dna.push('A');
            }
            ('A', 'T') | ('G', 'G') | ('C', 'T') | ('T', 'A') | ('T', 'C') => {
                dna.push('G');
            }
            ('A', 'G') | ('G', 'A') | ('C', 'C') => dna.push('C'),
            ('G', 'C') | ('C', 'G') | ('T', 'T') => dna.push('T'),
            _ => unreachable!(),
        }
    }

    writeln!(out, "{}", dna[0]).unwrap();
}
