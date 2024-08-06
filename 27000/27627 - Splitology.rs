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

fn recursion(chars: &[char], left: usize, right: usize) -> bool {
    if left >= right {
        return true;
    } else if chars[left] != chars[right] {
        return false;
    } else {
        return recursion(chars, left + 1, right - 1);
    }
}

fn is_palindrome(chars: &[char]) -> bool {
    return recursion(chars, 0, chars.len() - 1);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    for i in 1..s.len() {
        if is_palindrome(&s[..i]) && is_palindrome(&s[i..]) {
            writeln!(
                out,
                "{} {}",
                &s[..i].iter().collect::<String>(),
                &s[i..].iter().collect::<String>()
            )
            .unwrap();
            return;
        }
    }

    writeln!(out, "NO").unwrap();
}
