use io::Write;
use std::{io, str, vec};

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
        let (h, w) = (scan.token::<usize>(), scan.token::<usize>());

        if h == 0 && w == 0 {
            break;
        }

        let mut keyboard = vec![vec![' '; w]; h];

        for i in 0..h {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                keyboard[i][j] = c;
            }
        }

        let s = scan.token::<String>();
        let mut pos_y = 0;
        let mut pos_x = 0;
        let mut ret = 0;

        for c in s.chars() {
            let mut pos = (0, 0);

            for i in 0..h {
                for j in 0..w {
                    if keyboard[i][j] == c {
                        pos = (i, j);
                    }
                }
            }

            ret += (pos.0 as i32 - pos_y as i32).abs() + (pos.1 as i32 - pos_x as i32).abs() + 1;
            pos_y = pos.0;
            pos_x = pos.1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
