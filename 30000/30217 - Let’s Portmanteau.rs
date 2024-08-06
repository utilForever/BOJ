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

    let first = scan.token::<String>();
    let second = scan.token::<String>();
    let first = first.chars().collect::<Vec<_>>();
    let second = second.chars().collect::<Vec<_>>();

    let mut idx_v1 = 1;
    let mut idx_v2 = (second.len() - 2) as i64;

    while idx_v1 < first.len() {
        if first[idx_v1] == 'a'
            || first[idx_v1] == 'e'
            || first[idx_v1] == 'i'
            || first[idx_v1] == 'o'
            || first[idx_v1] == 'u'
        {
            break;
        } else {
            idx_v1 += 1;
        }
    }

    while idx_v2 >= 0 {
        if second[idx_v2 as usize] == 'a'
            || second[idx_v2 as usize] == 'e'
            || second[idx_v2 as usize] == 'i'
            || second[idx_v2 as usize] == 'o'
            || second[idx_v2 as usize] == 'u'
        {
            break;
        } else {
            idx_v2 -= 1;
        }
    }

    let ret = if idx_v1 < first.len() && idx_v2 >= 0 {
        first[..idx_v1].iter().collect::<String>()
            + &second[idx_v2 as usize..].iter().collect::<String>()
    } else if idx_v1 < first.len() {
        first[..=idx_v1].iter().collect::<String>()
            + &second[(idx_v2 + 1) as usize..].iter().collect::<String>()
    } else if idx_v2 >= 0 {
        first[..idx_v1].iter().collect::<String>()
            + &second[idx_v2 as usize..].iter().collect::<String>()
    } else {
        first.iter().collect::<String>() + "o" + &second.iter().collect::<String>()
    };

    writeln!(out, "{ret}").unwrap();
}
