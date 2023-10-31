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

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();
    let mut rainbow_lowercase = vec![0; 7];
    let mut rainbow_uppercase = vec![0; 7];

    for ch in s.chars() {
        match ch {
            'r' => rainbow_lowercase[0] += 1,
            'o' => rainbow_lowercase[1] += 1,
            'y' => rainbow_lowercase[2] += 1,
            'g' => rainbow_lowercase[3] += 1,
            'b' => rainbow_lowercase[4] += 1,
            'i' => rainbow_lowercase[5] += 1,
            'v' => rainbow_lowercase[6] += 1,
            'R' => rainbow_uppercase[0] += 1,
            'O' => rainbow_uppercase[1] += 1,
            'Y' => rainbow_uppercase[2] += 1,
            'G' => rainbow_uppercase[3] += 1,
            'B' => rainbow_uppercase[4] += 1,
            'I' => rainbow_uppercase[5] += 1,
            'V' => rainbow_uppercase[6] += 1,
            _ => (),
        }
    }

    let can_make_rainbow_lowercase = rainbow_lowercase.iter().all(|&x| x > 0);
    let can_make_rainbow_uppercase = rainbow_uppercase.iter().all(|&x| x > 0);

    writeln!(
        out,
        "{}",
        if can_make_rainbow_lowercase && can_make_rainbow_uppercase {
            "YeS"
        } else if can_make_rainbow_lowercase {
            "yes"
        } else if can_make_rainbow_uppercase {
            "YES"
        } else {
            "NO!"
        }
    )
    .unwrap();
}
