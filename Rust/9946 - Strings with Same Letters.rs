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

    let mut idx = 1;

    loop {
        let (word1, word2) = (scan.token::<String>(), scan.token::<String>());

        if word1 == "END" && word2 == "END" {
            break;
        }

        let mut alphabet1 = [0; 26];
        let mut alphabet2 = [0; 26];

        for c in word1.chars() {
            alphabet1[c as usize - 'a' as usize] += 1;
        }

        for c in word2.chars() {
            alphabet2[c as usize - 'a' as usize] += 1;
        }

        writeln!(
            out,
            "Case {idx}: {}",
            if alphabet1 == alphabet2 {
                "same"
            } else {
                "different"
            }
        )
        .unwrap();

        idx += 1;
    }
}
