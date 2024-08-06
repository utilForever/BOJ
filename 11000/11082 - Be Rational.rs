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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    if !s.contains(&'.') {
        writeln!(out, "{}/1", s.iter().collect::<String>()).unwrap();
        return;
    }

    let pos = s.iter().position(|&x| x == '.').unwrap();
    let number = s[0..pos].iter().collect::<String>().parse::<i64>().unwrap();
    let s = &s[pos - 1..];

    if s[2..].contains(&'(') {
        // Indefinitely case
        let pos = s[2..].iter().position(|&x| x == '(').unwrap();
        let prefix = if pos == 0 {
            0
        } else {
            s[2..pos + 2]
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        };
        let repeating = s[pos + 3..s.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let len_prefix = pos as u32;
        let len_repeating = (s.len() - 3 - pos) as u32;

        let numerator = prefix * 10i64.pow(len_repeating - 1) + repeating - prefix;
        let denominator = 10i64.pow(len_prefix + len_repeating - 1) - 10i64.pow(len_prefix);
        let gcd = gcd(numerator, denominator);

        writeln!(
            out,
            "{}/{}",
            number * (denominator / gcd) + numerator / gcd,
            denominator / gcd
        )
        .unwrap();
    } else {
        // Finitely case
        let digit = s[2..].len();
        let numerator = s[2..].iter().collect::<String>().parse::<i64>().unwrap();
        let denominator = 10i64.pow(digit as u32);
        let gcd = gcd(numerator, denominator);

        writeln!(
            out,
            "{}/{}",
            number * (denominator / gcd) + numerator / gcd,
            denominator / gcd
        )
        .unwrap();
    }
}
