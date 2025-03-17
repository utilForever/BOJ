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

    let word_kangaroo = "kangaroo";
    let word_kiwibird = "kiwibird";
    let mut cnt_kangaroo = [0; 26];
    let mut cnt_kiwibird = [0; 26];

    for c in word_kangaroo.chars() {
        cnt_kangaroo[c as usize - 'a' as usize] += 1;
    }

    for c in word_kiwibird.chars() {
        cnt_kiwibird[c as usize - 'a' as usize] += 1;
    }

    let s = scan.token::<String>();
    let mut score_kangaroo = 0;
    let mut score_kiwibird = 0;

    for c in s.chars() {
        let c = c.to_ascii_lowercase();
        
        score_kangaroo += cnt_kangaroo[c as usize - 'a' as usize];
        score_kiwibird += cnt_kiwibird[c as usize - 'a' as usize];
    }

    writeln!(
        out,
        "{}",
        if score_kangaroo > score_kiwibird {
            "Kangaroos"
        } else if score_kangaroo < score_kiwibird {
            "Kiwis"
        } else {
            "Feud continues"
        }
    )
    .unwrap();
}
