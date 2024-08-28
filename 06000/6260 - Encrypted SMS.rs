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

    let keypads = vec![
        vec!['A', 'B', 'C'],
        vec!['D', 'E', 'F'],
        vec!['G', 'H', 'I'],
        vec!['J', 'K', 'L'],
        vec!['M', 'N', 'O'],
        vec!['P', 'Q', 'R', 'S'],
        vec!['T', 'U', 'V'],
        vec!['W', 'X', 'Y', 'Z'],
    ];

    loop {
        let s = scan.token::<String>();

        if s == "#" {
            break;
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut ret = String::new();

        for (i, c) in s.iter().enumerate() {
            let is_lowercase = c.is_lowercase();
            let c = c.to_uppercase().next().unwrap();
            let pos_keypad = keypads.iter().position(|x| x.contains(&c)).unwrap();
            let mut pos_key = keypads[pos_keypad].iter().position(|x| *x == c).unwrap();
            let repeat = i + 1;

            for _ in 0..repeat {
                if pos_key == 0 {
                    pos_key = keypads[pos_keypad].len() - 1;
                } else {
                    pos_key -= 1;
                }
            }

            ret.push(if is_lowercase {
                keypads[pos_keypad][pos_key].to_lowercase().next().unwrap()
            } else {
                keypads[pos_keypad][pos_key]
            });
        }

        writeln!(out, "{ret}").unwrap();
    }
}
