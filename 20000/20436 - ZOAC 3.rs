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

    let keyboard = [
        ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ' '],
        ['z', 'x', 'c', 'v', 'b', 'n', 'm', ' ', ' ', ' '],
    ];
    let hangeul_consonant = [
        'q', 'w', 'e', 'r', 't', 'a', 's', 'd', 'f', 'g', 'z', 'x', 'c', 'v',
    ];
    let (sl, sr) = (scan.token::<char>(), scan.token::<char>());
    let (mut pos_sl, mut pos_sr) = ((0, 0), (0, 0));

    for i in 0..3 {
        for j in 0..10 {
            if keyboard[i][j] == sl {
                pos_sl = (i as i32, j as i32);
            }

            if keyboard[i][j] == sr {
                pos_sr = (i as i32, j as i32);
            }
        }
    }

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut ret = 0;

    for c in s {
        let mut pos_c = (0, 0);

        for i in 0..3 {
            for j in 0..10 {
                if keyboard[i][j] == c {
                    pos_c = (i as i32, j as i32);
                }
            }
        }

        if hangeul_consonant.contains(&c) {
            let dist = (pos_sl.0 - pos_c.0).abs() + (pos_sl.1 - pos_c.1).abs();
            pos_sl = pos_c;
            ret += dist + 1;
        } else {
            let dist = (pos_sr.0 - pos_c.0).abs() + (pos_sr.1 - pos_c.1).abs();
            pos_sr = pos_c;
            ret += dist + 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
