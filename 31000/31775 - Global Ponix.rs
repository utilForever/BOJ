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

    let mut strings = vec![String::new(); 3];

    for i in 0..3 {
        strings[i] = scan.token::<String>();
    }

    let (mut cnt_l, mut cnt_k, mut cnt_p) = (0, 0, 0);

    for string in strings {
        if string.starts_with("l") {
            cnt_l += 1;
        } else if string.starts_with("k") {
            cnt_k += 1;
        } else if string.starts_with("p") {
            cnt_p += 1;
        }
    }

    writeln!(
        out,
        "{}",
        if cnt_l == 1 && cnt_k == 1 && cnt_p == 1 {
            "GLOBAL"
        } else {
            "PONIX"
        }
    )
    .unwrap();
}
