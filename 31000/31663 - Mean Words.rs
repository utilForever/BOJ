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

    let n = scan.token::<usize>();
    let mut words = vec![String::new(); n];

    for i in 0..n {
        words[i] = scan.token::<String>();
    }

    let len_max = words.iter().map(|x| x.len()).max().unwrap();
    let mut chars = vec![vec![' '; len_max]; n];

    for i in 0..n {
        for (j, c) in words[i].chars().enumerate() {
            chars[i][j] = c;
        }
    }

    let mut ret = String::new();

    for j in 0..len_max {
        let mut val = 0;
        let mut cnt = 0;

        for i in 0..n {
            if chars[i][j] == ' ' {
                continue;
            }

            val += chars[i][j] as i64 - 'a' as i64;
            cnt += 1;
        }

        ret.push(((val / cnt) as u8 + 'a' as u8) as char);
    }

    writeln!(out, "{ret}").unwrap();
}
