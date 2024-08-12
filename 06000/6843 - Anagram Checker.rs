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

    let s = scan.line().trim().to_string();
    let t = scan.line().trim().to_string();
    let mut alphabet_s = vec![0; 26];
    let mut alphabet_t = vec![0; 26];

    for c in s.chars() {
        if c.is_whitespace() {
            continue;
        }

        alphabet_s[(c as u8 - 'A' as u8) as usize] += 1;
    }

    for c in t.chars() {
        if c.is_whitespace() {
            continue;
        }

        alphabet_t[(c as u8 - 'A' as u8) as usize] += 1;
    }

    writeln!(
        out,
        "{}",
        if alphabet_s == alphabet_t {
            "Is an anagram."
        } else {
            "Is not an anagram."
        }
    )
    .unwrap();
}
