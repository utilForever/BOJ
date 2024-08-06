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

// Reference: BOI 2003 Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (s, h, v) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let n = scan.token::<usize>();
    let mut cubes = vec![(0.0, 0.0); n];

    for i in 0..n {
        cubes[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    // Initialize value of minimum and maximum
    let mut left = v / s;
    let mut right = h;

    // Get the height of the cube by binary search
    while left <= right {
        let mid = (left + right) / 2.0;
        let mut ret_volume = 0.0;

        for &cube in cubes.iter() {
            // Calculate the height according to the density of the cube
            let mut cur_height = if cube.1 >= 1.0 {
                cube.0
            } else {
                cube.0 * cube.1
            };

            // Calculate the height of the cube that is submerged in the water
            // NOTE: Compare cur_height and mid becuase mid is the height of the water to check
            // NOTE: Then, compare cube.0 - cur_height and h - mid
            cur_height = cur_height.min(mid).max(mid + cube.0 - h);

            // Calculate the volume of the cube that is submerged in the water
            ret_volume += cur_height * cube.0 * cube.0;
        }

        // Compare the calculated volume and the assumed volume
        if ret_volume + v > mid * s {
            left = mid + 0.00001;
        } else {
            right = mid - 0.00001;
        }
    }

    writeln!(out, "{:.5}", left).unwrap();
}
