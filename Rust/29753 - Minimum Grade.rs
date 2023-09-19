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

    let (n, x) = (scan.token::<i64>(), scan.token::<f64>());
    let x = (x * 100.0).round() as i64;
    let mut sum_grades = 0;
    let mut credits = 0;

    for _ in 0..n - 1 {
        let (credit, letter_grade) = (scan.token::<i64>(), scan.token::<String>());

        sum_grades += credit
            * match letter_grade.as_str() {
                "A+" => 450,
                "A0" => 400,
                "B+" => 350,
                "B0" => 300,
                "C+" => 250,
                "C0" => 200,
                "D+" => 150,
                "D0" => 100,
                "F" => 0,
                _ => unreachable!(),
            };
        credits += credit;
    }

    let l = scan.token::<i64>();

    let calculate = |grade: &str| -> i64 {
        let num_grade = match grade {
            "A+" => 450,
            "A0" => 400,
            "B+" => 350,
            "B0" => 300,
            "C+" => 250,
            "C0" => 200,
            "D+" => 150,
            "D0" => 100,
            "F" => 0,
            _ => unreachable!(),
        };

        (sum_grades + l * num_grade) / (credits + l)
    };

    if calculate("A+") <= x {
        writeln!(out, "impossible").unwrap();
        return;
    }

    writeln!(
        out,
        "{}",
        if calculate("F") > x {
            "F"
        } else if calculate("D0") > x {
            "D0"
        } else if calculate("D+") > x {
            "D+"
        } else if calculate("C0") > x {
            "C0"
        } else if calculate("C+") > x {
            "C+"
        } else if calculate("B0") > x {
            "B0"
        } else if calculate("B+") > x {
            "B+"
        } else if calculate("A0") > x {
            "A0"
        } else {
            "A+"
        }
    )
    .unwrap();
}
