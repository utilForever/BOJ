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

    let mut words = vec![String::new(); 5];
    let mut length_max = 0;

    for i in 0..5 {
        words[i] = scan.token::<String>();

        if words[i].len() > length_max {
            length_max = words[i].len();
        }
    }

    let mut ret = vec![vec![' '; length_max]; 5];

    for i in 0..5 {
        for (j, c) in words[i].chars().enumerate() {
            ret[i][j] = c;
        }
    }

    for i in 0..length_max {
        for j in 0..5 {
            if ret[j][i] != ' ' {
                write!(out, "{}", ret[j][i]).unwrap();
            }
        }
    }

    writeln!(out).unwrap();
}
