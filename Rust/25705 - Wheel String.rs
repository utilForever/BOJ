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

    let _ = scan.token::<usize>();
    let s = scan.token::<String>();
    let _ = scan.token::<usize>();
    let t = scan.token::<String>();

    let mut s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut ret = 0;
    let mut flag = true;

    for (i, ch) in t.iter().enumerate() {
        ret += 1;

        if i != 0 {
            s.rotate_left(1);
        }

        let pos = s.iter().position(|&x| x == *ch);

        if pos.is_none() {
            flag = false;
            break;
        } else {
            let pos = pos.unwrap() as i64;
            ret += pos;

            s.rotate_left(pos as usize);
        }
    }

    writeln!(out, "{}", if flag { ret } else { -1 }).unwrap();
}
