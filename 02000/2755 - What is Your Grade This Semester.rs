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
    let mut total_credit = 0.0;
    let mut total_grade = 0.0;

    for _ in 0..n {
        let (_, credit, grade) = (
            scan.token::<String>(),
            scan.token::<i64>() as f64,
            scan.token::<String>(),
        );

        match grade.as_str() {
            "A+" => total_grade += 4.3 * credit,
            "A0" => total_grade += 4.0 * credit,
            "A-" => total_grade += 3.7 * credit,
            "B+" => total_grade += 3.3 * credit,
            "B0" => total_grade += 3.0 * credit,
            "B-" => total_grade += 2.7 * credit,
            "C+" => total_grade += 2.3 * credit,
            "C0" => total_grade += 2.0 * credit,
            "C-" => total_grade += 1.7 * credit,
            "D+" => total_grade += 1.3 * credit,
            "D0" => total_grade += 1.0 * credit,
            "D-" => total_grade += 0.7 * credit,
            "F" => total_grade += 0.0 * credit,
            _ => continue,
        }

        total_credit += credit;
    }

    writeln!(out, "{:.2}", total_grade / total_credit + 0.000000000001).unwrap();
}
