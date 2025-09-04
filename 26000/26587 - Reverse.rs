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

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let words = line.split_whitespace().collect::<Vec<&str>>();
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut starts_with_vowel = Vec::new();

        for (i, word) in words.iter().enumerate() {
            if vowels.contains(&word.chars().next().unwrap()) {
                starts_with_vowel.push(i);
            }
        }

        let mut idx = starts_with_vowel.len() - 1;

        for i in 0..words.len() {
            if starts_with_vowel.contains(&i) {
                write!(out, "{}", words[starts_with_vowel[idx]]).unwrap();
                idx -= 1;
            } else {
                write!(out, "{}", words[i]).unwrap();
            }

            if i != words.len() - 1 {
                write!(out, " ").unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
