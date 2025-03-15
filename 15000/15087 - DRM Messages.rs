use io::Write;
use std::{io, str, vec};

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

    let drm_message = scan.token::<String>();

    // Divide
    let (left, right) = drm_message.split_at(drm_message.len() / 2);

    // Rotate
    let rotate = |s: &str| -> String {
        let mut sum = 0;

        for c in s.chars() {
            sum += c as i64 - 'A' as i64;
        }

        let mut ret = String::new();

        for c in s.chars() {
            let c = c as u8 - 'A' as u8;
            let c_new = (c as i64 + sum) % 26;

            ret.push((c_new as u8 + 'A' as u8) as char);
        }

        ret
    };

    let rotated_left = rotate(left);
    let rotated_right = rotate(right);

    // Merge
    let mut ret = String::new();

    for (left, right) in rotated_left.chars().zip(rotated_right.chars()) {
        let left = left as u8 - 'A' as u8;
        let right = right as u8 - 'A' as u8;
        let sum = (left + right) % 26;

        ret.push((sum + 'A' as u8) as char);
    }

    writeln!(out, "{ret}").unwrap();
}
