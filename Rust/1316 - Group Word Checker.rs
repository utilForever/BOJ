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
    let mut num_group_word = 0;

    for _ in 0..n {
        let word = scan.token::<String>();
        let word = word.chars().collect::<Vec<_>>();
        let mut num_count = vec![0; 26];
        let mut is_group_word = true;

        let mut j = 0;

        while j < word.len() {
            if num_count[word[j] as usize - 'a' as usize] == 0 {
                let alphabet = word[j];
                num_count[word[j] as usize - 'a' as usize] = 1;

                while j < word.len() && word[j] == alphabet {
                    j += 1;
                }

                if j < word.len() {
                    j -= 1;
                }
            } else {
                is_group_word = false;
                break;
            }

            j += 1;
        }

        if is_group_word {
            num_group_word += 1;
        }
    }

    writeln!(out, "{}", num_group_word).unwrap();
}
