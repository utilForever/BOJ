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

    let is_inout = |typing: &str| -> bool {
        let left = "fdsa".chars().collect::<Vec<_>>();
        let right = "jkl;".chars().collect::<Vec<_>>();
        let mut idx_left = 0;
        let mut idx_right = 0;

        for c in typing.chars() {
            if idx_left < 4 && c == left[idx_left] {
                idx_left += 1;
            } else if idx_right < 4 && c == right[idx_right] {
                idx_right += 1;
            }
        }

        idx_left == left.len() && idx_right == right.len()
    };
    let is_outin = |typing: &str| -> bool {
        let left = "asdf".chars().collect::<Vec<_>>();
        let right = ";lkj".chars().collect::<Vec<_>>();
        let mut idx_left = 0;
        let mut idx_right = 0;

        for c in typing.chars() {
            if idx_left < 4 && c == left[idx_left] {
                idx_left += 1;
            } else if idx_right < 4 && c == right[idx_right] {
                idx_right += 1;
            }
        }

        idx_left == left.len() && idx_right == right.len()
    };
    let is_stairs = |typing: &str| -> bool { typing == "asdfjkl;" };
    let is_reverse = |typing: &str| -> bool { typing == ";lkjfdsa" };

    let typing = scan.token::<String>();

    writeln!(
        out,
        "{}",
        if is_inout(&typing) {
            "in-out"
        } else if is_outin(&typing) {
            "out-in"
        } else if is_stairs(&typing) {
            "stairs"
        } else if is_reverse(&typing) {
            "reverse"
        } else {
            "molu"
        }
    )
    .unwrap();
}
