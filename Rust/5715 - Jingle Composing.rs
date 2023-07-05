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

    loop {
        let notes = scan.token::<String>();

        if notes == "*" {
            break;
        }

        let notes = notes
            .split('/')
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut ret = 0;

        for note in notes {
            let mut duration = 0.0;

            for identifier in note {
                duration += match identifier {
                    'W' => 1.0,
                    'H' => 1.0 / 2.0,
                    'Q' => 1.0 / 4.0,
                    'E' => 1.0 / 8.0,
                    'S' => 1.0 / 16.0,
                    'T' => 1.0 / 32.0,
                    'X' => 1.0 / 64.0,
                    _ => unreachable!(),
                }
            }

            if duration == 1.0 {
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
