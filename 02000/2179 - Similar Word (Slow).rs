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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut words = vec![(String::new(), 0); n];

    for i in 0..n {
        let word = scan.token::<String>();
        words[i] = (word, i);
    }

    let mut ret = (0, usize::MAX, usize::MAX);

    for i in 0..n {
        for j in i + 1..n {
            let (word1, idx1) = &words[i];
            let (word2, idx2) = &words[j];
            let mut cnt_prefix = 0;

            for (c1, c2) in word1.chars().zip(word2.chars()) {
                if c1 == c2 {
                    cnt_prefix += 1;
                } else {
                    break;
                }
            }

            let (idx1, idx2) = (idx1.min(idx2), idx1.max(idx2));

            if cnt_prefix > ret.0
                || (cnt_prefix == ret.0 && idx1 < &ret.1)
                || (cnt_prefix == ret.0 && idx1 == &ret.1 && idx2 < &ret.2)
            {
                ret = (cnt_prefix, *idx1, *idx2);
            }
        }
    }

    let ret1 = words
        .iter()
        .find(|(_, idx)| *idx == ret.1)
        .map(|(word, _)| word.clone())
        .unwrap_or(String::new());
    let ret2 = words
        .iter()
        .find(|(_, idx)| *idx == ret.2)
        .map(|(word, _)| word.clone())
        .unwrap_or(String::new());

    writeln!(out, "{ret1}").unwrap();
    writeln!(out, "{ret2}").unwrap();
}
