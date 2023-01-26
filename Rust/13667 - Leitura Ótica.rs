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

    loop {
        let n = scan.token::<i64>();

        if n == 0 {
            break;
        }

        for _ in 0..n {
            let mut colors = [0; 5];

            for i in 0..5 {
                colors[i] = scan.token::<i64>();
            }

            let cnt = colors.iter().filter(|&&x| x <= 127).count();

            writeln!(
                out,
                "{}",
                if cnt == 1 {
                    match colors.iter().position(|&x| x <= 127).unwrap() {
                        0 => "A",
                        1 => "B",
                        2 => "C",
                        3 => "D",
                        4 => "E",
                        _ => unreachable!(),
                    }
                } else {
                    "*"
                }
            )
            .unwrap();
        }
    }
}
