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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn next_string(prefix: &[char]) -> Option<String> {
    let mut ret = prefix.to_vec();

    for i in (0..ret.len()).rev() {
        if ret[i] < 'Z' {
            ret[i] = (ret[i] as u8 + 1) as char;

            for j in i + 1..ret.len() {
                ret[j] = 'A';
            }

            return Some(ret.iter().collect());
        }
    }

    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut names = vec![String::new(); n];

        for i in 0..n {
            names[i] = scan.token::<String>();
        }

        names.sort_unstable();

        let left = names[n / 2 - 1].chars().collect::<Vec<_>>();
        let right = names[n / 2].chars().collect::<Vec<_>>();

        for i in 0..left.len() {
            let cand = if i < left.len() - 1 {
                match next_string(&left[..=i]) {
                    Some(s) => s,
                    None => continue,
                }
            } else {
                left.iter().collect::<String>()
            };

            if cand.chars().collect::<Vec<_>>() < right {
                writeln!(out, "{cand}").unwrap();
                break;
            }
        }
    }
}
