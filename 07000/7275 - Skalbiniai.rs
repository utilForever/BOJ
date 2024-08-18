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

    let (n, k, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut washing_machines = vec![Vec::new(); k];

    for i in 0..k {
        let g = scan.token::<usize>();
        washing_machines[i] = vec![0; g];

        for j in 0..g {
            washing_machines[i][j] = scan.token::<usize>();
        }
    }

    let mut laundries = vec![0; n];

    for i in 0..n {
        laundries[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    for machine in washing_machines {
        let mut cnt = 0;

        for cloth in machine {
            cnt += laundries[cloth - 1];
        }

        ret += if cnt % m == 0 { cnt / m } else { cnt / m + 1 };
    }

    writeln!(out, "{ret}").unwrap();
}
