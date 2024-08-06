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

    let mut picture = vec![vec![' '; 10001]; 5];

    for i in 0..5 {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            picture[i][j] = c;
        }
    }

    for i in 0..10001 {
        if picture[0][i] == 'o' {
            picture[0][i] = '.';
            picture[1][i] = 'o';
            picture[2][i] = 'm';
            picture[3][i] = 'l';
            picture[4][i] = 'n';
        } else if picture[1][i] == 'o' {
            picture[0][i] = 'o';
            picture[1][i] = 'w';
            picture[2][i] = 'l';
            picture[3][i] = 'n';
            picture[4][i] = '.';
        }
    }

    for i in 0..5 {
        for j in 0..10001 {
            if picture[i][j] == ' ' {
                break;
            }

            write!(out, "{}", picture[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
