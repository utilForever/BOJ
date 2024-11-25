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

    let (n, k) = (scan.token::<i64>(), scan.token::<usize>());

    if n == 0 {
        for _ in 0..3 {
            writeln!(out).unwrap();
        }
        
        return;
    }

    let s = scan.token::<String>();
    let mut boxes = vec![String::new(); 3];

    for c in s.chars() {
        if boxes.iter().all(|b| b.len() >= k) {
            break;
        }

        let mut idx = match c {
            's' => 0,
            'r' => 1,
            'p' => 2,
            _ => unreachable!(),
        };

        loop {
            if boxes[idx].len() < k {
                boxes[idx].push(c);
                break;
            } else {
                idx = (idx + 1) % 3;
            }
        }
    }

    for i in 0..3 {
        writeln!(out, "{}", boxes[i]).unwrap();
    }
}
