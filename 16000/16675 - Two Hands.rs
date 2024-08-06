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

    let (m_l, m_r, t_l, t_r) = (
        scan.token::<char>(),
        scan.token::<char>(),
        scan.token::<char>(),
        scan.token::<char>(),
    );
    let convert = |c: char| match c {
        'R' => 0,
        'S' => 1,
        'P' => 2,
        _ => unreachable!(),
    };

    let m_l = convert(m_l);
    let m_r = convert(m_r);
    let t_l = convert(t_l);
    let t_r = convert(t_r);

    writeln!(
        out,
        "{}",
        if m_l == m_r && ((m_l + 2) % 3 == t_l || (m_l + 2) % 3 == t_r) {
            "TK"
        } else if t_l == t_r && ((t_l + 2) % 3 == m_l || (t_l + 2) % 3 == m_r) {
            "MS"
        } else {
            "?"
        }
    )
    .unwrap();
}
