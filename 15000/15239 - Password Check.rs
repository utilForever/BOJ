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

    let t = scan.token::<i64>();
    let symbols = [
        '+', '_', ')', '(', '*', '&', '^', '%', '$', '#', '@', '!', '.', '/', ',', ';', '{', '}',
    ];

    for _ in 0..t {
        let n = scan.token::<i64>();
        let password = scan.token::<String>().chars().collect::<Vec<_>>();

        if n < 12 {
            writeln!(out, "invalid").unwrap();
            continue;
        }

        let exist_lowercase = password.iter().any(|&c| c.is_ascii_lowercase());
        let exist_uppercase = password.iter().any(|&c| c.is_ascii_uppercase());
        let exist_digit = password.iter().any(|&c| c.is_ascii_digit());
        let exist_symbol = password.iter().any(|&c| symbols.contains(&c));

        writeln!(
            out,
            "{}",
            if exist_lowercase && exist_uppercase && exist_digit && exist_symbol {
                "valid"
            } else {
                "invalid"
            }
        )
        .unwrap();
    }
}
