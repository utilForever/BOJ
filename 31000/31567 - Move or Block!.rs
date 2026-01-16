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
    let board = scan.token::<String>().chars().collect::<Vec<_>>();
    let pos = board.iter().position(|&c| c == 'O').unwrap();
    let cnt_dot = board.iter().filter(|&&c| c == '.').count();

    let mut left = pos;
    let mut right = pos;

    while left > 0 && board[left - 1] == '.' {
        left -= 1;
    }

    while right < n - 1 && board[right + 1] == '.' {
        right += 1;
    }

    let len = right - left + 1;
    let pos = pos - left;
    let cnt = cnt_dot - (len - 1);

    writeln!(
        out,
        "{}",
        if len == 3 && pos == 1 && (cnt % 2 == 0) {
            "yunsu"
        } else if pos == 0 || pos == len - 1 {
            "mingyu"
        } else if (pos == 1 || pos == len - 2) && ((len + cnt) % 2 == 0) {
            "mingyu"
        } else {
            "draw"
        }
    )
    .unwrap();
}
