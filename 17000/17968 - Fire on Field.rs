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

    let n = scan.token::<usize>();
    let mut ret = vec![0; n + 1];

    if n == 0 || n == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    ret[0] = 1;
    ret[1] = 1;

    for i in 2..=n {
        let mut num = 1;

        loop {
            let mut check = true;
            let mut idx = 1;

            ret[i] = num;

            while i as i64 - 2 * idx as i64 >= 0 {
                if ret[i] - ret[i - idx] == ret[i - idx] - ret[i - 2 * idx] {
                    check = false;
                    break;
                }

                idx += 1;
            }

            if check {
                break;
            }

            num += 1;
        }
    }

    writeln!(out, "{}", ret[n]).unwrap();
}
