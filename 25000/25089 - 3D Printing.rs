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

    let t = scan.token::<i32>();
    let mut cyan = [0; 3];
    let mut magenta = [0; 3];
    let mut yellow = [0; 3];
    let mut black = [0; 3];

    for i in 1..=t {
        for j in 0..3 {
            cyan[j] = scan.token::<i32>();
            magenta[j] = scan.token::<i32>();
            yellow[j] = scan.token::<i32>();
            black[j] = scan.token::<i32>();
        }

        let mut cyan_min = *cyan.iter().min().unwrap();
        let mut magenta_min = *magenta.iter().min().unwrap();
        let mut yellow_min = *yellow.iter().min().unwrap();
        let mut black_min = *black.iter().min().unwrap();
        let sum = cyan_min + magenta_min + yellow_min + black_min;

        if sum < 1_000_000 {
            writeln!(out, "Case #{i}: IMPOSSIBLE").unwrap();
            continue;
        }

        let mut diff = sum - 1_000_000;

        let val = diff.min(cyan_min - 1);
        diff -= val;
        cyan_min -= val;

        let val = diff.min(magenta_min - 1);
        diff -= val;
        magenta_min -= val;

        let val = diff.min(yellow_min - 1);
        diff -= val;
        yellow_min -= val;

        let val = diff.min(black_min - 1);
        black_min -= val;

        writeln!(
            out,
            "Case #{i}: {cyan_min} {magenta_min} {yellow_min} {black_min}",
        )
        .unwrap();
    }
}
