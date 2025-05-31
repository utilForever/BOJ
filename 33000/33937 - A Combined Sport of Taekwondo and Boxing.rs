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

    let a = scan.token::<String>().chars().collect::<Vec<_>>();
    let b = scan.token::<String>().chars().collect::<Vec<_>>();

    let a_first_vowel = a
        .iter()
        .position(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));
    let b_first_vowel = b
        .iter()
        .position(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));

    if a_first_vowel.is_none() || b_first_vowel.is_none() {
        writeln!(out, "no such exercise").unwrap();
        return;
    }

    let a_first_vowel = a_first_vowel.unwrap();
    let b_first_vowel = b_first_vowel.unwrap();

    let a_first_consonant = a[a_first_vowel + 1..]
        .iter()
        .position(|&c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));
    let b_first_consonant = b[b_first_vowel + 1..]
        .iter()
        .position(|&c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));

    if a_first_consonant.is_none() || b_first_consonant.is_none() {
        writeln!(out, "no such exercise").unwrap();
        return;
    }

    let a_first_consonant = a_first_consonant.unwrap();
    let b_first_consonant = b_first_consonant.unwrap();

    let ret_a = &a[..a_first_vowel + 1 + a_first_consonant];
    let ret_b = &b[..b_first_vowel + 1 + b_first_consonant];

    writeln!(
        out,
        "{}{}",
        ret_a.iter().collect::<String>(),
        ret_b.iter().collect::<String>()
    )
    .unwrap();
}
