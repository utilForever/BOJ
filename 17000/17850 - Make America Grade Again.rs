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

    let (l, h, p, e, n) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut point_lab = 0;
    let mut score_lab = 0;
    let mut point_homework = 0;
    let mut score_homework = 0;
    let mut point_project = 0;
    let mut score_project = 0;
    let mut point_exam = 0;
    let mut score_exam = 0;

    for _ in 0..n {
        let (cat, _, score) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let scores = score.split('/').collect::<Vec<_>>();
        let (point, score) = (
            scores[0].parse::<i64>().unwrap(),
            scores[1].parse::<i64>().unwrap(),
        );

        match cat.as_str() {
            "Lab" => {
                point_lab += point;
                score_lab += score;
            }
            "Hw" => {
                point_homework += point;
                score_homework += score;
            }
            "Proj" => {
                point_project += point;
                score_project += score;
            }
            "Exam" => {
                point_exam += point;
                score_exam += score;
            }
            _ => unreachable!(),
        }
    }

    let ret = l as f64 * (point_lab as f64 / score_lab as f64)
        + h as f64 * (point_homework as f64 / score_homework as f64)
        + p as f64 * (point_project as f64 / score_project as f64)
        + e as f64 * (point_exam as f64 / score_exam as f64);

    writeln!(out, "{}", ret as i64).unwrap();
}
