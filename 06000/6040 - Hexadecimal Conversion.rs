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

    let s = scan.token::<String>();
    let mut bits = String::new();

    for c in s.chars() {
        bits.push_str(match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unreachable!(),
        });
    }

    let bits = bits.chars().collect::<Vec<_>>();
    let mut idx = bits.len() as i64 - 1;
    let mut ret = String::new();

    while idx >= 0 {
        if idx >= 2 {
            let num = (bits[idx as usize] as u8 - b'0') * 1
                + (bits[idx as usize - 1] as u8 - b'0') * 2
                + (bits[idx as usize - 2] as u8 - b'0') * 4;
            ret.push((num + b'0') as char);
            idx -= 3;
        } else if idx == 1 {
            let num =
                (bits[idx as usize] as u8 - b'0') * 1 + (bits[idx as usize - 1] as u8 - b'0') * 2;
            ret.push((num + b'0') as char);
            idx -= 2;
        } else if idx == 0 {
            let num = bits[idx as usize] as u8 - b'0';
            ret.push((num + b'0') as char);
            idx -= 1;
        }
    }

    let mut ret = ret.chars().rev().collect::<String>();
    ret = ret.trim_start_matches('0').to_string();

    if ret.is_empty() {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{ret}").unwrap();
}
