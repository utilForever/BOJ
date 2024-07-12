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

    loop {
        let votes = scan.token::<String>();

        if votes == "#" {
            break;
        }

        let (mut cnt_yes, mut cnt_no, mut cnt_absent) = (0, 0, 0);

        for vote in votes.chars() {
            match vote {
                'Y' => cnt_yes += 1,
                'N' => cnt_no += 1,
                'A' => cnt_absent += 1,
                _ => (),
            }
        }

        writeln!(
            out,
            "{}",
            if cnt_absent >= (votes.len() + 1) / 2 {
                "need quorum"
            } else if cnt_yes > cnt_no {
                "yes"
            } else if cnt_yes < cnt_no {
                "no"
            } else {
                "tie"
            }
        )
        .unwrap();
    }
}
