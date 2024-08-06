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

    let mirror_chars = ['b', 'd', 'p', 'q', 'i', 'o', 'v', 'w', 'x'];

    loop {
        let s = scan.token::<String>();

        if s == "#" {
            break;
        }

        let mut s = s.chars().collect::<Vec<_>>();
        let cnt = s.iter().filter(|&c| mirror_chars.contains(c)).count();

        if cnt != s.len() {
            writeln!(out, "INVALID").unwrap();
            continue;
        }

        for c in s.iter_mut() {
            match c {
                'b' => *c = 'd',
                'd' => *c = 'b',
                'p' => *c = 'q',
                'q' => *c = 'p',
                'i' => *c = 'i',
                'o' => *c = 'o',
                'v' => *c = 'v',
                'w' => *c = 'w',
                'x' => *c = 'x',
                _ => unreachable!(),
            }
        }

        s.reverse();

        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
    }
}
