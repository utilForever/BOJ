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

    let s = scan.token::<String>();
    let t = scan.token::<String>();

    let mut alphabet_s = vec![0; 26];
    let mut alphabet_t = vec![0; 26];
    let mut asterisk = 0;

    for c in s.chars() {
        alphabet_s[(c as u8 - b'a') as usize] += 1;
    }

    for c in t.chars() {
        if c == '*' {
            asterisk += 1;
        } else {
            alphabet_t[(c as u8 - b'a') as usize] += 1;
        }
    }

    let mut ret = true;

    for i in 0..26 {
        if alphabet_s[i] < alphabet_t[i] {
            ret = false;
            break;
        }
    }

    if !ret {
        writeln!(out, "N").unwrap();
        return;
    }

    let mut diff = 0;

    for i in 0..26 {
        diff += alphabet_s[i] - alphabet_t[i];
    }

    writeln!(out, "{}", if diff <= asterisk { "A" } else { "N" }).unwrap();
}
