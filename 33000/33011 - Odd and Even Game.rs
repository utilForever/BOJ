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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut cards = vec![0; n];

        for i in 0..n {
            cards[i] = scan.token::<i64>();
        }

        let (mut cnt_odd, mut cnt_even) = (0, 0);

        for &card in cards.iter() {
            if card % 2 == 0 {
                cnt_even += 1;
            } else {
                cnt_odd += 1;
            }
        }

        writeln!(
            out,
            "{}",
            if (cnt_odd % 2 == 1 && cnt_odd > cnt_even) || (cnt_even % 2 == 1 && cnt_even > cnt_odd)
            {
                "amsminn"
            } else {
                "heeda0528"
            }
        )
        .unwrap();
    }
}
