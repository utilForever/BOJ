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

    let mut colors_target = [(0, 0, 0); 16];

    for i in 0..16 {
        colors_target[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    loop {
        let color = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if color == (-1, -1, -1) {
            break;
        }

        let mut ret = f64::MAX;
        let mut ret_color = (0, 0, 0);

        for color_target in colors_target.iter() {
            let dist = ((color_target.0 - color.0).pow(2)
                + (color_target.1 - color.1).pow(2)
                + (color_target.2 - color.2).pow(2)) as f64;

            if dist < ret {
                ret = dist;
                ret_color = *color_target;
            }
        }

        writeln!(
            out,
            "({},{},{}) maps to ({},{},{})",
            color.0, color.1, color.2, ret_color.0, ret_color.1, ret_color.2
        )
        .unwrap();
    }
}
