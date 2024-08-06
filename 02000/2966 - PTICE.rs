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

    let _ = scan.token::<i64>();
    let answers = scan.token::<String>();
    let answers = answers.chars().collect::<Vec<_>>();

    let answer_adrian = ['A', 'B', 'C'];
    let answer_bruno = ['B', 'A', 'B', 'C'];
    let answer_goran = ['C', 'C', 'A', 'A', 'B', 'B'];
    let mut ret_adrian = 0;
    let mut ret_bruno = 0;
    let mut ret_goran = 0;

    for (idx, answer) in answers.iter().enumerate() {
        if answer_adrian[idx % 3] == *answer {
            ret_adrian += 1;
        }

        if answer_bruno[idx % 4] == *answer {
            ret_bruno += 1;
        }

        if answer_goran[idx % 6] == *answer {
            ret_goran += 1;
        }
    }

    let max = ret_adrian.max(ret_bruno).max(ret_goran);

    writeln!(out, "{max}").unwrap();

    if max == ret_adrian {
        writeln!(out, "Adrian").unwrap();
    }

    if max == ret_bruno {
        writeln!(out, "Bruno").unwrap();
    }

    if max == ret_goran {
        writeln!(out, "Goran").unwrap();
    }
}
