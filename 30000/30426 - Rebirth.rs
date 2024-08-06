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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut dimensions = vec![false; n];
    let mut dimensions_unstable = vec![false; n];
    let mut moves_accepted = vec![0; k];
    let mut moves_wrong_answer = vec![0; k];

    dimensions[m] = true;

    for i in 0..k {
        moves_accepted[i] = scan.token::<usize>();
        moves_wrong_answer[i] = scan.token::<usize>();
    }

    let l = scan.token::<usize>();

    for _ in 0..l {
        let x = scan.token::<usize>();
        dimensions_unstable[x] = true;
    }

    if dimensions_unstable[m] {
        writeln!(out, "dystopia").unwrap();
        return;
    }

    for i in 0..k {
        let mut can_move = vec![false; n];

        for j in 0..n {
            if dimensions[j] {
                let next_accepted = (j + moves_accepted[i]) % n;
                let next_wrong_answer = (j + moves_wrong_answer[i]) % n;

                if !dimensions_unstable[next_accepted] {
                    can_move[next_accepted] = true;
                }

                if !dimensions_unstable[next_wrong_answer] {
                    can_move[next_wrong_answer] = true;
                }
            }
        }

        for j in 0..n {
            dimensions[j] = can_move[j];
        }
    }

    writeln!(out, "{}", if dimensions[0] { "utopia" } else { "dystopia" }).unwrap();
}
