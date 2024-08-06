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

    let (s, p) = (scan.token::<usize>(), scan.token::<usize>());
    let dna = scan.token::<String>();
    let dna = dna.chars().collect::<Vec<_>>();
    let (a, c, g, t) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut cnt_a = 0;
    let mut cnt_c = 0;
    let mut cnt_g = 0;
    let mut cnt_t = 0;
    let mut ret = 0;

    for i in 0..p {
        match dna[i] {
            'A' => cnt_a += 1,
            'C' => cnt_c += 1,
            'G' => cnt_g += 1,
            'T' => cnt_t += 1,
            _ => {}
        }
    }

    if cnt_a >= a && cnt_c >= c && cnt_g >= g && cnt_t >= t {
        ret += 1;
    }

    for i in p..s {
        match dna[i] {
            'A' => cnt_a += 1,
            'C' => cnt_c += 1,
            'G' => cnt_g += 1,
            'T' => cnt_t += 1,
            _ => {}
        }

        match dna[i - p] {
            'A' => cnt_a -= 1,
            'C' => cnt_c -= 1,
            'G' => cnt_g -= 1,
            'T' => cnt_t -= 1,
            _ => {}
        }

        if cnt_a >= a && cnt_c >= c && cnt_g >= g && cnt_t >= t {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
