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
    let mut teams = vec![String::new(); n];

    for i in 0..n {
        teams[i] = scan.token::<String>();
    }

    for team in teams {
        let team = team.chars().collect::<Vec<_>>();
        let cnt_uppercase = team.iter().filter(|&c| c.is_uppercase()).count();
        let cnt_lowercase = team.iter().filter(|&c| c.is_lowercase()).count();

        if cnt_uppercase > cnt_lowercase {
            continue;
        }

        if team.len() > 10 {
            continue;
        }

        if team.iter().filter(|&c| c.is_numeric()).count() == team.len() {
            continue;
        }

        writeln!(out, "{}", team.iter().collect::<String>()).unwrap();
        break;
    }
}
