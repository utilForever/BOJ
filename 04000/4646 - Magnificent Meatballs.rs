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

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut meatballs = vec![0; n];

        for i in 0..n {
            meatballs[i] = scan.token::<i64>();
        }

        let (mut pos_sam, mut pos_ella) = (0, n - 1);
        let (mut sum_sam, mut sum_ella) = (0, 0);

        while pos_sam <= pos_ella {
            if sum_sam < sum_ella {
                sum_sam += meatballs[pos_sam];
                pos_sam += 1;
            } else {
                sum_ella += meatballs[pos_ella];
                pos_ella -= 1;
            }
        }

        if sum_sam == sum_ella {
            writeln!(
                out,
                "Sam stops at position {} and Ella stops at position {}.",
                pos_ella + 1,
                pos_sam + 1
            )
            .unwrap();
        } else {
            writeln!(out, "No equal partitioning.").unwrap();
        }
    }
}
