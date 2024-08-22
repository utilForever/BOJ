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

    let _ = scan.token::<String>();
    let whites = scan.token::<String>();
    let _ = scan.token::<String>();
    let blacks = scan.token::<String>();

    let whites = whites
        .split(",")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let blacks = blacks
        .split(",")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut ret = vec![vec![' '; 8]; 8];

    for white in whites {
        match white[0] {
            'K' | 'Q' | 'R' | 'B' | 'N' => {
                let col = (white[1] as u8 - b'a') as usize;
                let row = (8 - (white[2] as u8 - b'0')) as usize;

                ret[row][col] = white[0];
            }
            _ => {
                let col = (white[0] as u8 - b'a') as usize;
                let row = (8 - (white[1] as u8 - b'0')) as usize;

                ret[row][col] = 'P';
            }
        }
    }

    for black in blacks {
        match black[0] {
            'K' | 'Q' | 'R' | 'B' | 'N' => {
                let col = (black[1] as u8 - b'a') as usize;
                let row = (8 - (black[2] as u8 - b'0')) as usize;

                ret[row][col] = black[0].to_ascii_lowercase();
            }
            _ => {
                let col = (black[0] as u8 - b'a') as usize;
                let row = (8 - (black[1] as u8 - b'0')) as usize;

                ret[row][col] = 'p';
            }
        }
    }

    writeln!(out, "+---+---+---+---+---+---+---+---+").unwrap();

    for i in 0..8 {
        write!(out, "|").unwrap();

        for j in 0..8 {
            write!(out, "{}", if (i + j) % 2 == 0 { '.' } else { ':' }).unwrap();

            if ret[i][j] == ' ' {
                write!(out, "{}", if (i + j) % 2 == 0 { '.' } else { ':' }).unwrap();
            } else {
                write!(out, "{}", ret[i][j]).unwrap();
            }

            write!(out, "{}|", if (i + j) % 2 == 0 { '.' } else { ':' }).unwrap();
        }

        writeln!(out).unwrap();
        writeln!(out, "+---+---+---+---+---+---+---+---+").unwrap();
    }
}
