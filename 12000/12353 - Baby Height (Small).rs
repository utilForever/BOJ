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

    for i in 1..=t {
        let (gender, height_mother, height_father) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let mut height_mother = height_mother
            .split("'")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut height_father = height_father
            .split("'")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        height_mother[1].pop();
        height_father[1].pop();

        let (feet_mother, inch_mother) = (
            height_mother[0].parse::<i64>().unwrap(),
            height_mother[1].parse::<i64>().unwrap(),
        );
        let (feet_father, inch_father) = (
            height_father[0].parse::<i64>().unwrap(),
            height_father[1].parse::<i64>().unwrap(),
        );

        let height_mother = feet_mother * 12 + inch_mother;
        let height_father = feet_father * 12 + inch_father;
        let mut height_total = height_mother + height_father + if gender == "B" { 5 } else { -5 };

        let is_odd = height_total % 2 == 1;

        if is_odd {
            height_total -= 1;
        }

        let height_total = (height_total / 2) - 4 + if is_odd { 1 } else { 0 };
        let (feet_from, inch_from) = (height_total / 12, height_total % 12);

        let height_total = height_total + 8 - if is_odd { 1 } else { 0 };
        let (feet_to, inch_to) = (height_total / 12, height_total % 12);

        writeln!(
            out,
            "Case #{i}: {feet_from}'{inch_from}\" to {feet_to}'{inch_to}\""
        )
        .unwrap();
    }
}
