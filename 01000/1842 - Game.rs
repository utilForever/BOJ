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

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut pawns = vec![0; n + 2];
    let mut converted_pawns = vec![0; n + 2];

    for i in 1..=n {
        pawns[i] = scan.token::<usize>();
    }

    pawns[n + 1] = m - 1;

    if pawns[n] == m - 1 {
        let mut ret = 0;

        for i in (1..=n).rev() {
            if pawns[i] - pawns[i - 1] != 1 {
                break;
            }

            ret += 1;
        }

        writeln!(out, "{}", ret + 1).unwrap();
        return;
    }

    let mut top_pos = 0;
    let mut now_pos = 0;

    for i in (1..=n).rev() {
        if pawns[i + 1] - pawns[i] != 1 {
            converted_pawns[top_pos] = now_pos;
            top_pos += 1;
            now_pos = 0;

            if pawns[i + 1] - pawns[i] > 2 {
                top_pos += 2 - ((pawns[i + 1] - pawns[i]) & 1);
            }
        }

        now_pos += 1;
    }

    let mut ans = 0;
    let mut ret = 0;

    converted_pawns[top_pos] = now_pos;

    for i in 1..=top_pos {
        if i % 2 == 1 {
            ans ^= converted_pawns[i];
        }
    }

    for i in 1..=top_pos {
        match i % 2 {
            0 => {
                if (converted_pawns[i - 1] ^ ans) > converted_pawns[i - 1]
                    && (converted_pawns[i - 1] ^ ans) <= converted_pawns[i] + converted_pawns[i - 1]
                {
                    ret += 1;
                }
            }
            1 => {
                if (converted_pawns[i] ^ ans) < converted_pawns[i] {
                    ret += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
