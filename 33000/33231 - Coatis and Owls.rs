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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut coatis = vec![0; n];
    let mut owls = vec![0; m];

    for i in (0..n).rev() {
        coatis[i] = scan.token::<i64>();
    }

    for i in 0..m {
        owls[i] = scan.token::<i64>();
    }

    let mut idx_coati = 0;
    let mut idx_owl = 0;

    while idx_coati < n && idx_owl < m {
        let coati = coatis[idx_coati];
        let owl = owls[idx_owl];

        if coati == owl {
            coatis[idx_coati] = 0;
            owls[idx_owl] = 0;
            idx_coati += 1;
            idx_owl += 1;
        } else if coati > owl {
            let lose = if (owl * owl) % coati == 0 {
                (owl * owl) / coati
            } else {
                (owl * owl) / coati + 1
            };

            coatis[idx_coati] -= lose;
            owls[idx_owl] = 0;
            idx_owl += 1;
        } else {
            let lose = if (coati * coati) % owl == 0 {
                (coati * coati) / owl
            } else {
                (coati * coati) / owl + 1
            };

            owls[idx_owl] -= lose;
            coatis[idx_coati] = 0;
            idx_coati += 1;
        }
    }

    if idx_coati == n && idx_owl == m {
        writeln!(out, "stalemate").unwrap();
    } else if idx_coati == n {
        writeln!(out, "owls").unwrap();
        writeln!(out, "{}", owls.iter().sum::<i64>()).unwrap();
    } else {
        writeln!(out, "coatis").unwrap();
        writeln!(out, "{}", coatis.iter().sum::<i64>()).unwrap();
    }
}
