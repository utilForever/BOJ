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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (r, p, d) = (
            scan.token::<usize>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        let mut ingredients = Vec::with_capacity(r);

        for _ in 0..r {
            let (name, weight, percentage) = (
                scan.token::<String>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            ingredients.push((name, weight, percentage));
        }

        let main_pos = ingredients
            .iter()
            .position(|(_, _, p)| (*p - 100.0).abs() < 1e-6)
            .unwrap();
        let main_scaled = ingredients[main_pos].1 * (d / p);

        writeln!(out, "Recipe # {i}").unwrap();

        for i in 0..r {
            let scaled = ingredients[i].2 * main_scaled / 100.0;
            writeln!(out, "{} {:.1}", ingredients[i].0, scaled).unwrap();
        }

        writeln!(out, "{}", "-".repeat(40)).unwrap();
    }
}
