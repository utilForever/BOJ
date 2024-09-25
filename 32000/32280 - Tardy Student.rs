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

    let n = scan.token::<usize>();
    let mut times = vec![(0, false); n + 1];

    for i in 0..=n {
        let (time, is_student) = (
            scan.token::<String>(),
            if scan.token::<String>() == "student" {
                true
            } else {
                false
            },
        );
        let time = time
            .split(":")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        times[i] = (time[0] * 60 + time[1], is_student);
    }

    let time_teacher = scan.token::<String>();
    let time_teacher = time_teacher
        .split(":")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let time_teacher = time_teacher[0] * 60 + time_teacher[1];

    times.sort();

    let pos = times.iter().position(|&x| !x.1).unwrap();
    let mut ret = 0;

    for i in pos + 1..=n {
        if times[i].0 >= time_teacher {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
