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
        let word1 = scan.token::<String>();

        if word1 == "#" {
            break;
        }

        let word1 = word1.chars().collect::<Vec<_>>();
        let word2 = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut diff = vec![0; word1.len()];

        for i in 0..word1.len() {
            diff[i] = ((word2[i] as u8 - b'a') as i32 - (word1[i] as u8 - b'a') as i32 + 26) % 26;
        }

        let word3 = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut ret = vec![0; word3.len()];

        for i in 0..word3.len() {
            ret[i] = ((word3[i] as u8 - b'a') as i32 + diff[i % word1.len()] + 26) % 26;
        }

        write!(
            out,
            "{} {} {} ",
            word1.iter().collect::<String>(),
            word2.iter().collect::<String>(),
            word3.iter().collect::<String>()
        )
        .unwrap();

        for i in 0..ret.len() {
            write!(out, "{}", (ret[i] as u8 + b'a') as char).unwrap();
        }

        writeln!(out).unwrap();
    }
}
