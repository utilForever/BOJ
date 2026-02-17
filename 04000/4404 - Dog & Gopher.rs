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

    let (gopher_x, gopher_y, dog_x, dog_y) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );

    let mut ret = None;

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let words = line.split_whitespace().collect::<Vec<_>>();
        let (hole_x, hole_y) = (
            words[0].parse::<f64>().unwrap(),
            words[1].parse::<f64>().unwrap(),
        );

        let dist_gopher = ((gopher_x - hole_x).powi(2) + (gopher_y - hole_y).powi(2)).sqrt();
        let dist_dog = ((dog_x - hole_x).powi(2) + (dog_y - hole_y).powi(2)).sqrt();

        if 2.0 * dist_gopher < dist_dog && ret.is_none() {
            ret = Some((hole_x, hole_y));
        }
    }

    match ret {
        Some((x, y)) => writeln!(
            out,
            "The gopher can escape through the hole at ({:.3},{:.3}).",
            x, y
        )
        .unwrap(),
        None => writeln!(out, "The gopher cannot escape.").unwrap(),
    }
}
