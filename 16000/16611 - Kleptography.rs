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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let last_letters = scan.token::<String>().chars().collect::<Vec<_>>();
    let ciphertext = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut diff = vec![0; m];
    let mut ret = vec![' '; m];

    for i in (m - n..m).rev() {
        ret[i] = last_letters[i - m + n];
        diff[i] = (ciphertext[i] as i32 - ret[i] as i32 + 26) % 26;
    }

    for i in (0..m - n).rev() {
        ret[i] = (diff[n + i] + 'a' as i32) as u8 as char;
        diff[i] = (ciphertext[i] as i32 - ret[i] as i32 + 26) % 26;
    }

    writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
}
