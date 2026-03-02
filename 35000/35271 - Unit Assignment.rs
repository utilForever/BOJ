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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut seats_limit = vec![0; m];
    let mut seats_occupied = vec![0; m];
    let mut ret = vec![-1; n];

    for i in 0..m {
        seats_limit[i] = scan.token::<i64>();
    }

    for i in 0..n {
        let mut priority = [0; 3];

        for j in 0..3 {
            priority[j] = scan.token::<usize>() - 1;
        }

        for j in 0..3 {
            let seat = priority[j];

            if seats_occupied[seat] < seats_limit[seat] {
                seats_occupied[seat] += 1;
                ret[i] = (seat + 1) as i64;
                break;
            }
        }
    }

    for i in 0..n {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
