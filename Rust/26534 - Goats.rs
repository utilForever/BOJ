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

    let mut player1 = [0; 6];
    let mut player2 = [0; 6];

    for i in 0..6 {
        player1[i] = scan.token::<i64>();
    }

    for i in 0..6 {
        player2[i] = scan.token::<i64>();
    }

    let mut cnt_player1 = 0;
    let mut cnt_player2 = 0;

    for i in 0..6 {
        for j in 0..6 {
            if player1[i] > player2[j] {
                cnt_player1 += 1;
            } else if player1[i] < player2[j] {
                cnt_player2 += 1;
            }
        }
    }

    if cnt_player1 + cnt_player2 == 0 {
        writeln!(out, "0.00000").unwrap();
        return;
    }

    writeln!(
        out,
        "{:.5}",
        cnt_player1 as f64 / (cnt_player1 + cnt_player2) as f64
    )
    .unwrap();
}
