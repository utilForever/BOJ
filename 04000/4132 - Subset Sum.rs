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

    let (n, m) = (scan.token::<u64>(), scan.token::<usize>());
    let mut sizes = vec![0; m];

    for i in 0..m {
        sizes[i] = scan.token::<u64>();
    }

    let mut ret: Option<u64> = None;

    for i in 0..(1 << m) {
        let mut sum = 0;

        for (j, size) in sizes.iter().enumerate() {
            if (i >> j) & 1 == 1 {
                sum += size;
            }
        }

        if sum >= n {
            match ret {
                Some(val) => ret = Some(val.min(sum)),
                None => ret = Some(sum),
            }
        }
    }

    match ret {
        Some(val) => writeln!(out, "{val}").unwrap(),
        None => writeln!(out, "IMPOSSIBLE").unwrap(),
    }
}
