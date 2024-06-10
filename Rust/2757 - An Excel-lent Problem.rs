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
        let cell = scan.token::<String>();

        if cell == "R0C0" {
            break;
        }

        let row = cell
            .chars()
            .skip(1)
            .take_while(|&c| c != 'C')
            .collect::<String>();
        let col = cell
            .chars()
            .skip_while(|&c| c != 'C')
            .skip(1)
            .collect::<String>();

        let mut col = col.parse::<i64>().unwrap();
        let mut col_converted = Vec::new();

        while col > 0 {
            let c = ((col - 1) % 26) as u8 + b'A';
            col_converted.push(c as char);

            col = (col - 1) / 26;
        }

        col_converted.reverse();

        writeln!(out, "{}{row}", col_converted.iter().collect::<String>()).unwrap();
    }
}
