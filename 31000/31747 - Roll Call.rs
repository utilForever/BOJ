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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut students = vec![0; n];

    for i in 0..n {
        students[i] = scan.token::<i64>();
    }

    let mut cnt_student_first = 0;
    let mut cnt_student_second = 0;
    let mut idx = 0;
    let mut ret = 0;

    while idx < n {
        while cnt_student_first + cnt_student_second < k && idx < n {
            if students[idx] == 1 {
                cnt_student_first += 1;
            } else {
                cnt_student_second += 1;
            }

            idx += 1;
        }

        if cnt_student_first > 0 && cnt_student_second > 0 {
            cnt_student_first -= 1;
            cnt_student_second -= 1;
            ret += 1;
        } else if cnt_student_first > 0 {
            cnt_student_first -= 1;
            ret += 1;
        } else if cnt_student_second > 0 {
            cnt_student_second -= 1;
            ret += 1;
        }
    }

    while cnt_student_first + cnt_student_second > 0 {
        if cnt_student_first > 0 && cnt_student_second > 0 {
            cnt_student_first -= 1;
            cnt_student_second -= 1;
            ret += 1;
        } else if cnt_student_first > 0 {
            cnt_student_first -= 1;
            ret += 1;
        } else if cnt_student_second > 0 {
            cnt_student_second -= 1;
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
