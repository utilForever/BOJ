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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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
    let mut cnt_students = 0;
    let mut cnt_faculty = 0;
    let mut cnt_visitors = 0;

    for _ in 0..n {
        let (type_person, direction, num) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );

        match type_person.as_str() {
            "STU" => match direction.as_str() {
                "IN" => cnt_students += num,
                "OUT" => cnt_students -= num,
                _ => unreachable!(),
            },
            "FAC" => match direction.as_str() {
                "IN" => cnt_faculty += num,
                "OUT" => cnt_faculty -= num,
                _ => unreachable!(),
            },
            "VIS" => match direction.as_str() {
                "IN" => cnt_visitors += num,
                "OUT" => cnt_visitors -= num,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    if cnt_students == 0 && cnt_faculty == 0 && cnt_visitors == 0 {
        writeln!(out, "NO STRAGGLERS").unwrap();
    } else {
        writeln!(out, "{}", cnt_students + cnt_faculty + cnt_visitors).unwrap();
    }
}
