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

    let (n, _) = (scan.token::<usize>(), scan.token::<usize>());
    let mut cnt_start = 0;
    let mut cnt_end = 0;
    let mut cnt_wall = 0;
    let mut cnt_virus_one = [0; 4];
    let mut cnt_virus_all = 0;
    let mut cnt_vaccine = 0;

    for _ in 0..n {
        let line = scan.token::<String>();

        for c in line.chars() {
            match c {
                '#' => cnt_wall += 1,
                'U' => cnt_virus_one[0] += 1,
                'D' => cnt_virus_one[1] += 1,
                'L' => cnt_virus_one[2] += 1,
                'R' => cnt_virus_one[3] += 1,
                'A' => cnt_virus_all += 1,
                'V' => cnt_vaccine += 1,
                'S' => cnt_start += 1,
                'E' => cnt_end += 1,
                _ => {}
            }
        }
    }

    if cnt_start != 1 || cnt_end != 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    if (cnt_wall <= 1 && cnt_virus_one.iter().sum::<i64>() <= 1)
        && cnt_virus_all == 0
        && cnt_vaccine == 0
    {
        writeln!(out, "1").unwrap();
        return;
    }

    if (cnt_wall >= 2 || cnt_virus_one.iter().sum::<i64>() >= 2)
        && cnt_virus_all == 0
        && cnt_vaccine == 0
    {
        writeln!(out, "2").unwrap();
        return;
    }

    if cnt_virus_all == 0 {
        writeln!(out, "3").unwrap();
        return;
    }

    writeln!(out, "4").unwrap();
}
