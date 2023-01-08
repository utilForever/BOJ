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

fn recursion(chars: &Vec<char>, left: usize, right: usize, cnt_recursion: &mut usize) -> i64 {
    *cnt_recursion += 1;

    if left >= right {
        return 1;
    } else if chars[left] != chars[right] {
        return 0;
    } else {
        return recursion(chars, left + 1, right - 1, cnt_recursion);
    }
}

fn is_palindrome(chars: &Vec<char>, cnt_recursion: &mut usize) -> i64 {
    return recursion(chars, 0, chars.len() - 1, cnt_recursion);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<char>>();

        let mut cnt_recursion = 0;
        let ret = is_palindrome(&s, &mut cnt_recursion);

        writeln!(out, "{ret} {cnt_recursion}").unwrap();
    }
}
