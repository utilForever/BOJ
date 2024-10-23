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

fn check_slump(s: &[char]) -> bool {
    if s.len() < 3 {
        return false;
    }

    if s[0] != 'D' && s[0] != 'E' {
        return false;
    }

    if s[1] != 'F' {
        return false;
    }

    let mut idx = 2;

    while idx < s.len() && s[idx] == 'F' {
        idx += 1;
    }

    let check = check_slump(&s[idx..]);

    if check {
        return true;
    }

    idx == s.len() - 1 && s[idx] == 'G'
}

fn check_slimp(s: &[char]) -> bool {
    if s.len() < 2 {
        return false;
    }

    if s[0] != 'A' {
        return false;
    }

    if s.len() == 2 {
        return s[1] == 'H';
    }

    if s[s.len() - 1] != 'C' {
        return false;
    }

    let mut ret = check_slump(&s[1..s.len() - 1]);

    if s[1] == 'B' {
        ret |= check_slimp(&s[2..s.len() - 1]);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut check = false;

        for i in 0..s.len() {
            let word1 = &s[..i];
            let word2 = &s[i..];

            let is_slimp = check_slimp(word1);
            let is_slump = check_slump(word2);

            if is_slimp && is_slump {
                check = true;
                break;
            }
        }

        writeln!(out, "{}", if check { "YES" } else { "NO" }).unwrap();
    }
}
