use std::{collections::HashSet, io, str};

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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();

    println!("late");

    if n % 2 == 1 {
        println!("orakGo");
        return;
    }

    let mut s = (3..=n).step_by(2).collect::<HashSet<_>>();
    let mut t = (2..=n + 1).step_by(2).collect::<HashSet<_>>();

    let mut arr = (0..=n).collect::<Vec<_>>();
    let mut irr = (0..=n).collect::<Vec<_>>();
    let mut brr = vec![false; n + 1];

    for i in 1..=n {
        if i == 1 {
            brr[1] = true;
            arr.swap(2, n);
            irr.swap(2, n);
        } else if i % 2 == 1 {
            let input = scan.line().trim().to_string();

            if input == "orakGo" {
                return;
            }

            let x = input.parse::<usize>().unwrap();

            if x % 2 == 1 {
                s.remove(&x);
            } else {
                t.remove(&x);
            }

            let x_pos = arr[x];
            let y = if x_pos == 0 { n } else { x_pos - 1 };
            let mut z = x_pos + 1;

            if z > n {
                z -= n;
            }

            if x_pos % 2 == 0 && (!brr[y] || !brr[z]) {
                println!("orakGo");
                return;
            }

            brr[x_pos] = true;

            irr.swap(y, z);
            let iy = irr[y];
            let iz = irr[z];
            arr.swap(iy, iz);
        } else {
            let x_choice = if s.is_empty() {
                let val = *t.iter().next().unwrap();
                t.remove(&val);
                val
            } else {
                let val = *s.iter().next().unwrap();
                s.remove(&val);
                val
            };

            println!("{x_choice}");

            let x_pos = arr[x_choice];
            let y = if x_pos == 0 { n } else { x_pos - 1 };
            let mut z = x_pos + 1;

            if z > n {
                z -= n;
            }

            brr[x_pos] = true;

            irr.swap(y, z);
            let iy = irr[y];
            let iz = irr[z];
            arr.swap(iy, iz);
        }
    }
}
