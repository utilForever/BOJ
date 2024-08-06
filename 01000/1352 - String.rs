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

fn backtrack(
    alphabets: &mut Vec<usize>,
    n: usize,
    sum: usize,
    idx_start: usize,
    idx_end: usize,
    idx_alphabet: usize,
) -> bool {
    if sum == n {
        return true;
    }

    if sum > n {
        return false;
    }

    for i in (idx_start..=idx_end + 1).rev() {
        if backtrack(alphabets, n, sum + i, i + 1, idx_end + i, idx_alphabet + 1) {
            alphabets[idx_alphabet] = i;
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut alphabets = vec![0; 26];

    let can_ideal = backtrack(&mut alphabets, n, 0, 1, 0, 0);

    if can_ideal {
        let mut ret = vec![' '; n];

        for i in 0..26 {
            if alphabets[i] > 0 {
                alphabets[i] -= 1;
                ret[alphabets[i]] = (i as u8 + 'A' as u8) as char;
            }
        }

        let mut pos = ret.iter().position(|&c| c == ' ');

        for i in 0..26 {
            while alphabets[i] > 0 {
                alphabets[i] -= 1;

                if let Some(position) = pos {
                    ret[position] = (i as u8 + 'A' as u8) as char;
                    pos = ret.iter().position(|&c| c == ' ');
                }
            }
        }

        writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
