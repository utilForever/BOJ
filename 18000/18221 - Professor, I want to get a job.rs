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

    let n = scan.token::<i64>();
    let mut pos_sungkyu = (0, 0);
    let mut pos_professor = (0, 0);
    let mut pos_students = Vec::new();

    for i in 0..n {
        for j in 0..n {
            let val = scan.token::<i64>();

            match val {
                1 => pos_students.push((i, j)),
                2 => pos_sungkyu = (i, j),
                5 => pos_professor = (i, j),
                _ => (),
            }
        }
    }

    let dist = (pos_sungkyu.0 - pos_professor.0).pow(2) + (pos_sungkyu.1 - pos_professor.1).pow(2);

    if dist < 25 {
        writeln!(out, "0").unwrap();
        return;
    }

    let top_left = (
        pos_professor.0.min(pos_sungkyu.0),
        pos_professor.1.min(pos_sungkyu.1),
    );
    let bottom_right = (
        pos_professor.0.max(pos_sungkyu.0),
        pos_professor.1.max(pos_sungkyu.1),
    );
    let mut cnt_students = 0;

    for pos_student in pos_students {
        if pos_student.0 >= top_left.0
            && pos_student.0 <= bottom_right.0
            && pos_student.1 >= top_left.1
            && pos_student.1 <= bottom_right.1
        {
            cnt_students += 1;
        }
    }

    writeln!(out, "{}", if cnt_students >= 3 { 1 } else { 0 }).unwrap();
}
