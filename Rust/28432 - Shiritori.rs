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
    let mut pos_question = 0;

    for i in 0..n {
        words[i] = scan.token::<String>();

        if words[i] == "?" {
            pos_question = i;
        }
    }

    let m = scan.token::<usize>();
    let mut candidates = vec![String::new(); m];

    for i in 0..m {
        candidates[i] = scan.token::<String>();
    }

    let mut char_start = None;
    let mut char_end = None;

    if pos_question > 0 {
        char_start = Some(words[pos_question - 1].chars().last().unwrap());
    }

    if pos_question < n - 1 {
        char_end = Some(words[pos_question + 1].chars().nth(0).unwrap());
    }

    let ret = candidates
        .iter()
        .filter(|&candidate| {
            if words.iter().any(|word| word == candidate) {
                return false;
            }

            true
        })
        .find(|&candidate| {
            if let Some(char_start) = char_start {
                if candidate.chars().nth(0).unwrap() != char_start {
                    return false;
                }
            }

            if let Some(char_end) = char_end {
                if candidate.chars().last().unwrap() != char_end {
                    return false;
                }
            }

            true
        })
        .unwrap();

    writeln!(out, "{ret}").unwrap();
}
