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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let scale = [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];

    loop {
        let line = scan.line().trim().to_string();

        if line == "***" {
            break;
        }

        let mut notes = line.split_whitespace().collect::<Vec<_>>();

        for note in notes.iter_mut() {
            *note = match *note {
                "Ab" => "G#",
                "Bb" => "A#",
                "Cb" => "B",
                "Db" => "C#",
                "Eb" => "D#",
                "Fb" => "E",
                "Gb" => "F#",
                "B#" => "C",
                "E#" => "F",
                _ => *note,
            }
        }

        let offset = scan.token::<i64>();

        for note in notes {
            let idx = scale.iter().position(|&s| s == note).unwrap();
            let idx_new = (idx as i64 + offset).rem_euclid(12);

            write!(out, "{} ", scale[idx_new as usize]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
