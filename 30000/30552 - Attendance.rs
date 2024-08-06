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
    let mut students = vec![String::new(); n];

    for i in 0..n {
        students[i] = scan.token::<String>();
    }

    let mut ret = Vec::new();

    students.windows(2).for_each(|x| {
        if x[0] != "Present!" && x[1] != "Present!" {
            ret.push(x[0].clone());
        }
    });

    if students[n - 1] != "Present!" {
        ret.push(students[n - 1].clone());
    }

    if ret.is_empty() {
        writeln!(out, "No Absences").unwrap();
    } else {
        ret.iter().for_each(|student| {
            writeln!(out, "{student}").unwrap();
        });
    }
}
