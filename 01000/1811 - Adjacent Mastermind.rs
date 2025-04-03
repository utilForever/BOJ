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
        let target = scan.token::<String>().chars().collect::<Vec<_>>();

        if target[0] == '#' {
            break;
        }

        let guess = scan.token::<String>().chars().collect::<Vec<_>>();

        let mut check_target = vec![false; target.len()];
        let mut check_guess = vec![false; guess.len()];
        let mut black = 0;
        let mut grey = 0;
        let mut white = 0;

        // Check Black
        for i in 0..target.len() {
            if target[i] == guess[i] {
                check_target[i] = true;
                check_guess[i] = true;
                black += 1;
            }
        }

        // Check Grey
        for i in 0..target.len() {
            if check_guess[i] {
                continue;
            }

            if i > 0 && guess[i] == target[i - 1] && !check_target[i - 1] {
                check_target[i - 1] = true;
                check_guess[i] = true;
                grey += 1;
            } else if i + 1 < target.len() && guess[i] == target[i + 1] && !check_target[i + 1] {
                check_target[i + 1] = true;
                check_guess[i] = true;
                grey += 1;
            }
        }

        // Check White
        for i in 0..target.len() {
            if check_guess[i] {
                continue;
            }

            for j in 0..target.len() {
                if check_target[j] {
                    continue;
                }

                if guess[i] == target[j] {
                    check_target[j] = true;
                    check_guess[i] = true;
                    white += 1;
                    break;
                }
            }
        }

        writeln!(
            out,
            "{}: {black} black, {grey} grey, {white} white",
            guess.iter().collect::<String>()
        )
        .unwrap();
    }
}
