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

    let alphabets = [
        3, 2, 1, 2, 3, 3, 2, 3, 3, 2, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 2, 2, 1,
    ];

    let (a, b) = (scan.token::<String>(), scan.token::<String>());
    let (a, b) = (a.chars().collect::<Vec<_>>(), b.chars().collect::<Vec<_>>());
    let mut combined = String::new();

    for i in 0..a.len() {
        let val_a = alphabets[(a[i] as u8 - b'A') as usize];
        let val_b = alphabets[(b[i] as u8 - b'A') as usize];

        combined.push((val_a as u8 + b'0') as char);
        combined.push((val_b as u8 + b'0') as char);
    }

    while combined.len() > 2 {
        let mut combined_new = String::new();
        let combined_chars = combined.chars().collect::<Vec<_>>();

        combined_chars.windows(2).for_each(|w| {
            let val_a = w[0] as u8 - '0' as u8;
            let val_b = w[1] as u8 - '0' as u8;

            if val_a + val_b >= 10 {
                combined_new.push((val_a + val_b - 10 + b'0') as char);
            } else {
                combined_new.push((val_a + val_b + b'0') as char);
            }
        });

        combined = combined_new;
    }

    writeln!(out, "{combined}").unwrap();
}
