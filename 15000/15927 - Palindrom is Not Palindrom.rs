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

fn process_manachers(text: &str) -> Vec<usize> {
    let mut s = String::from('*');

    for c in text.chars() {
        s.push(c);
        s.push('*');
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut ret = vec![0; s.len()];
    let mut r = 0;
    let mut c = 0;

    for i in 0..s.len() {
        ret[i] = if r < i { 0 } else { ret[2 * c - i].min(r - i) };

        while i as i64 - ret[i] as i64 - 1 >= 0
            && i + ret[i] + 1 < s.len()
            && s[i - ret[i] - 1] == s[i + ret[i] + 1]
        {
            ret[i] += 1;
        }

        if r < i + ret[i] {
            r = i + ret[i];
            c = i;
        }
    }

    ret
}

// Reference: http://www.secmem.org/blog/2019/03/10/Manacher/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let ret = process_manachers(&s);
    let palindrom_max = *ret.iter().max().unwrap();
    let is_all_same = s.chars().all(|c| c == s.chars().next().unwrap());

    writeln!(
        out,
        "{}",
        if is_all_same {
            -1
        } else if palindrom_max == s.len() {
            s.len() as i64 - 1
        } else {
            s.len() as i64
        }
    )
    .unwrap();
}
