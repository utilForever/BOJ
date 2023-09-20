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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut ret = s.clone();

    if s[0] == '_'
        || s[s.len() - 1] == '_'
        || s[0].is_uppercase()
        || s.windows(2).any(|w| (w[0] == '_' && w[1] == '_'))
    {
        writeln!(out, "Error!").unwrap();
        return;
    }

    if s.iter().any(|&c| c.is_uppercase()) && s.iter().any(|&c| c == '_') {
        writeln!(out, "Error!").unwrap();
        return;
    }

    if s.iter().any(|&c| c == '_') {
        let mut idx = 0;

        while idx < ret.len() {
            if ret[idx] == '_' {
                ret.remove(idx);
                ret[idx] = ret[idx].to_ascii_uppercase();
            }

            idx += 1;
        }
    } else {
        let mut idx = 0;

        while idx < ret.len() {
            if ret[idx].is_uppercase() {
                ret[idx] = ret[idx].to_ascii_lowercase();
                ret.insert(idx, '_');
            }

            idx += 1;
        }
    }

    writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
}
