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

    let alphabets = [
        ('A', "000000"),
        ('B', "001111"),
        ('C', "010011"),
        ('D', "011100"),
        ('E', "100110"),
        ('F', "101001"),
        ('G', "110101"),
        ('H', "111010"),
    ];

    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let mut ret = String::new();

    for i in 0..n {
        let word = s[i * 6..(i + 1) * 6]
            .to_string()
            .chars()
            .collect::<Vec<_>>();
        let mut diffs = [0; 8];

        for (idx, alphabet) in alphabets.iter().enumerate() {
            let num = alphabet.1.chars().collect::<Vec<_>>();
            let mut diff = 0;

            for j in 0..6 {
                if word[j] != num[j] {
                    diff += 1;
                }
            }

            diffs[idx] = diff;
        }

        let zero_diff = diffs.iter().filter(|&&x| x == 0).count();
        let one_diff = diffs.iter().filter(|&&x| x == 1).count();

        if zero_diff == 1 {
            ret.push(alphabets[diffs.iter().position(|&x| x == 0).unwrap()].0);
        } else if one_diff == 1 {
            ret.push(alphabets[diffs.iter().position(|&x| x == 1).unwrap()].0);
        } else {
            writeln!(out, "{}", i + 1).unwrap();
            return;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
