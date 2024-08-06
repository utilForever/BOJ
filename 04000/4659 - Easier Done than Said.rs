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

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn is_consonant(c: char) -> bool {
    !is_vowel(c)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let password = scan.token::<String>();

        if password == "end" {
            break;
        }

        let password = password.chars().collect::<Vec<_>>();
        let check1 = password.iter().any(|c| is_vowel(*c));
        let check2 = password.windows(3).all(|w| {
            let is_all_vowel = w.iter().all(|c| is_vowel(*c));
            let is_all_consonant = w.iter().all(|c| is_consonant(*c));

            !is_all_vowel && !is_all_consonant
        });
        let check3 = password
            .windows(2)
            .all(|w| w[0] != w[1] || w[0] == 'e' || w[0] == 'o');

        writeln!(
            out,
            "<{}> is {}.",
            password.iter().collect::<String>(),
            if check1 && check2 && check3 {
                "acceptable"
            } else {
                "not acceptable"
            }
        )
        .unwrap();
    }
}
