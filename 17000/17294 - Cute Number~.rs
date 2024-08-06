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

    let nums = scan
        .token::<String>()
        .chars()
        .map(|x| x as u8 - b'0')
        .collect::<Vec<_>>();

    if nums.len() == 1 {
        writeln!(out, "◝(⑅•ᴗ•⑅)◜..°♡ 뀌요미!!").unwrap();
        return;
    }

    let diff = nums[1] - nums[0];

    for i in 2..nums.len() {
        if nums[i] - nums[i - 1] != diff {
            writeln!(out, "흥칫뿡!! <(￣ ﹌ ￣)>").unwrap();
            return;
        }
    }

    writeln!(out, "◝(⑅•ᴗ•⑅)◜..°♡ 뀌요미!!").unwrap();
}
