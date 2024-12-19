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

    let mut idx = 1;

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        if idx > 1 {
            writeln!(out).unwrap();
        }

        let mut names = vec![String::new(); n];
        let mut messages = vec![vec![' '; n - 1]; n];

        for i in 0..n {
            names[i] = scan.token::<String>();

            for j in 0..n - 1 {
                messages[i][j] = scan.token::<char>();
            }
        }

        writeln!(out, "Group {idx}").unwrap();

        if messages.iter().flatten().all(|&c| c == 'P') {
            writeln!(out, "Nobody was nasty").unwrap();
        } else {
            for i in 0..n {
                for j in 0..n - 1 {
                    if messages[i][j] == 'N' {
                        let idx_a = if i >= j + 1 { i - j - 1 } else { i + n - j - 1 };
                        let idx_b = i;

                        writeln!(out, "{} was nasty about {}", names[idx_a], names[idx_b]).unwrap();
                    }
                }
            }
        }

        idx += 1;
    }
}
