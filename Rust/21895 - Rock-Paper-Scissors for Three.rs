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

    let _ = scan.token::<usize>();
    let robot1 = scan.token::<String>().chars().collect::<Vec<_>>();
    let robot2 = scan.token::<String>().chars().collect::<Vec<_>>();

    for (r1, r2) in robot1.iter().zip(robot2.iter()) {
        let mut combined = [r1, r2];
        combined.sort();

        match combined[0] {
            'P' => {
                write!(
                    out,
                    "{}",
                    if *combined[1] == 'P' {
                        "S"
                    } else if *combined[1] == 'R' {
                        "P"
                    } else {
                        "S"
                    }
                )
                .unwrap();
            }
            'R' => {
                write!(out, "{}", if *combined[1] == 'R' { "P" } else { "R" }).unwrap();
            }
            'S' => {
                write!(out, "R").unwrap();
            }
            _ => unreachable!(),
        }
    }

    writeln!(out).unwrap();
}
