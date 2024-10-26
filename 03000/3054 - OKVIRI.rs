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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let frame_peter_pan = vec![
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '.', '#', '.'],
        vec!['#', '.', 'P', '.', '#'],
        vec!['.', '#', '.', '#', '.'],
        vec!['.', '.', '#', '.', '.'],
    ];
    let frame_wendy = vec![
        vec!['.', '.', '*', '.', '.'],
        vec!['.', '*', '.', '*', '.'],
        vec!['*', '.', 'W', '.', '*'],
        vec!['.', '*', '.', '*', '.'],
        vec!['.', '.', '*', '.', '.'],
    ];

    let mut ret = vec![vec![' '; 5 + 4 * (s.len() - 1)]; 5];

    // First, fill "Peter Pan" frame
    for i in 0..s.len() {
        if (i + 1) % 3 == 0 {
            continue;
        }

        for j in 0..5 {
            for k in 0..5 {
                ret[j][i * 4 + k] = frame_peter_pan[j][k];
            }
        }

        ret[2][i * 4 + 2] = s[i];
    }

    // Then, fill "Wendy" frame
    for i in 0..s.len() {
        if (i + 1) % 3 != 0 {
            continue;
        }

        for j in 0..5 {
            for k in 0..5 {
                ret[j][i * 4 + k] = frame_wendy[j][k];
            }
        }

        ret[2][i * 4 + 2] = s[i];
    }

    for i in 0..5 {
        for j in 0..5 + 4 * (s.len() - 1) {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
