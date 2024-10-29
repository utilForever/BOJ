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

    let s = scan.token::<String>();
    let mut nums = String::new();

    for c in s.chars() {
        if c.is_numeric() {
            nums.push(c);
        }
    }

    let pin = nums[nums.len() - 11..].chars().collect::<String>();

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let (mut year, mut month, mut day) = (String::new(), String::new(), String::new());

    if s[0] <= '1' || (s[0] == '2' && s[1] <= '4') {
        year.push('2');
        year.push('0');
    } else {
        year.push('1');
        year.push('9');
    }

    year.push(s[0]);
    year.push(s[1]);
    month.push(s[2]);
    month.push(s[3]);
    day.push(s[4]);
    day.push(s[5]);

    let s = scan.token::<String>();
    let pos_first_end = s.find("<<").unwrap();
    let pos_last_start = pos_first_end + s[pos_first_end..].find(char::is_alphabetic).unwrap();
    let pos_last_end = pos_last_start + s[pos_last_start..].find('<').unwrap();
    let (mut first, mut last) = (
        s[..pos_first_end].chars().collect::<Vec<_>>(),
        s[pos_last_start..pos_last_end].chars().collect::<Vec<_>>(),
    );

    for i in 1..first.len() {
        first[i].make_ascii_lowercase();
    }

    for i in 1..last.len() {
        last[i].make_ascii_lowercase();
    }

    let (first, last) = (
        first.iter().collect::<String>(),
        last.iter().collect::<String>(),
    );

    writeln!(out, "Ime: {first}").unwrap();
    writeln!(out, "Prezime: {last}").unwrap();
    writeln!(out, "Datum rodjenja: {day}-{month}-{year}").unwrap();
    writeln!(out, "OIB: {pin}").unwrap();
}
