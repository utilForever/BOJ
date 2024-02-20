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

#[derive(Default, Clone, Copy)]
struct TrafficLight {
    pos: i64,
    time_red: i64,
    time_green: i64,
    is_red: bool,
    time_curr: i64,
}

impl TrafficLight {
    fn new(pos: i64, time_red: i64, time_green: i64) -> Self {
        Self {
            pos,
            time_red,
            time_green,
            is_red: true,
            time_curr: 0,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, l) = (scan.token::<usize>(), scan.token::<i64>());
    let mut lights = vec![TrafficLight::default(); n];

    for i in 0..n {
        let (pos, time_red, time_green) = (scan.token(), scan.token(), scan.token());
        lights[i] = TrafficLight::new(pos, time_red, time_green);
    }

    let mut pos = 0;
    let mut idx_light = 0;
    let mut ret = 0;

    loop {
        if pos == l {
            break;
        }

        if idx_light < lights.len() && lights[idx_light].pos == pos {
            if lights[idx_light].is_red {
                ret += 1;
            } else {
                pos += 1;
                idx_light += 1;
                ret += 1;
            }
        } else {
            pos += 1;
            ret += 1;
        }

        for light in lights.iter_mut() {
            light.time_curr += 1;

            if light.is_red && light.time_curr == light.time_red {
                light.is_red = false;
                light.time_curr = 0;
            } else if !light.is_red && light.time_curr == light.time_green {
                light.is_red = true;
                light.time_curr = 0;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
