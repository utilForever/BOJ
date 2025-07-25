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

    let mut nums = [0; 4];

    for i in 0..4 {
        nums[i] = scan.token::<i64>();
    }

    let mut diff = 0;

    if nums[0] != nums[3] {
        diff += 1;
    }

    if nums[1] != nums[2] {
        diff += 1;
    }

    if diff == 0 {
        writeln!(out, "JAH").unwrap();
        writeln!(out, "{} {} {} {}", nums[0], nums[1], nums[2], nums[3]).unwrap();
    } else if diff == 1 {
        writeln!(out, "JAH").unwrap();

        if nums[0] != nums[3] {
            nums[0] = nums[3];
        }

        if nums[1] != nums[2] {
            nums[1] = nums[2];
        }

        writeln!(out, "{} {} {} {}", nums[0], nums[1], nums[2], nums[3]).unwrap();
    } else {
        writeln!(out, "EI").unwrap();
    }
}
