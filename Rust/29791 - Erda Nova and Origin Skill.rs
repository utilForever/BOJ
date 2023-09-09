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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut times_erda_nova = vec![0; n];
    let mut times_origin_skills = vec![0; m];

    for i in 0..n {
        times_erda_nova[i] = scan.token::<i64>();
    }

    for i in 0..m {
        times_origin_skills[i] = scan.token::<i64>();
    }

    times_erda_nova.sort();
    times_origin_skills.sort();

    let mut ret_erda_nova = 0_i64;
    let mut prev_time_erda_nova = -100;

    for i in 0..n {
        if times_erda_nova[i] - prev_time_erda_nova >= 100 {
            ret_erda_nova += 1;
            prev_time_erda_nova = times_erda_nova[i];
        }
    }

    let mut ret_origin_skills = 0_i64;
    let mut prev_time_origin_skills = -360;

    for i in 0..m {
        if times_origin_skills[i] - prev_time_origin_skills >= 360 {
            ret_origin_skills += 1;
            prev_time_origin_skills = times_origin_skills[i];
        }
    }

    writeln!(out, "{ret_erda_nova} {ret_origin_skills}").unwrap();
}
