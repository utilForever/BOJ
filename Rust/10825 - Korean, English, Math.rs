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

#[derive(Default, Clone, Ord, PartialEq, Eq)]
struct Student {
    name: String,
    korean: i64,
    english: i64,
    math: i64,
}

impl Student {
    fn new(name: String, korean: i64, english: i64, math: i64) -> Self {
        Self {
            name,
            korean,
            english,
            math,
        }
    }
}

impl std::cmp::PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.korean == other.korean {
            if self.english == other.english {
                if self.math == other.math {
                    return self.name.partial_cmp(&other.name);
                }

                return other.math.partial_cmp(&self.math);
            }

            return self.english.partial_cmp(&other.english);
        }

        return other.korean.partial_cmp(&self.korean);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut students = vec![Student::default(); n];

    for i in 0..n {
        students[i] = Student::new(
            scan.token::<String>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    students.sort();

    for student in students {
        writeln!(out, "{}", student.name).unwrap();
    }
}
