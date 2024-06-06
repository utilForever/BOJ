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

    let music = scan.token::<String>();
    let note_first = music
        .split("|")
        .map(|x| x.chars().next().unwrap())
        .collect::<Vec<_>>();
    let mut cnt_a_minor = 0;
    let mut cnt_c_major = 0;

    for note in note_first.iter() {
        match note {
            'A' | 'D' | 'E' => cnt_a_minor += 1,
            'C' | 'F' | 'G' => cnt_c_major += 1,
            _ => (),
        }
    }

    writeln!(
        out,
        "{}",
        match cnt_a_minor.cmp(&cnt_c_major) {
            std::cmp::Ordering::Less => "C-major",
            std::cmp::Ordering::Equal => {
                let last = music.chars().last().unwrap();

                match last {
                    'A' | 'D' | 'E' => "A-minor",
                    'C' | 'F' | 'G' => "C-major",
                    _ => unreachable!(),
                }
            }
            std::cmp::Ordering::Greater => "A-minor",
        }
    )
    .unwrap();
}
