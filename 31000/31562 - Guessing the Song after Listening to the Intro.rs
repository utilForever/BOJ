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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut songs = vec![(String::new(), vec![' '; 7]); n];

    for i in 0..n {
        let (_, s) = (scan.token::<i64>(), scan.token::<String>());
        songs[i].0 = s;

        for j in 0..7 {
            songs[i].1[j] = scan.token::<char>();
        }
    }

    for _ in 0..m {
        let (b1, b2, b3) = (
            scan.token::<char>(),
            scan.token::<char>(),
            scan.token::<char>(),
        );
        let cnt = songs
            .iter()
            .filter(|s| s.1[0] == b1 && s.1[1] == b2 && s.1[2] == b3)
            .count();

        if cnt == 1 {
            writeln!(
                out,
                "{}",
                songs
                    .iter()
                    .find(|s| { s.1[0] == b1 && s.1[1] == b2 && s.1[2] == b3 })
                    .unwrap()
                    .0
            )
            .unwrap();
        } else if cnt > 1 {
            writeln!(out, "?").unwrap();
        } else {
            writeln!(out, "!").unwrap();
        }
    }
}
