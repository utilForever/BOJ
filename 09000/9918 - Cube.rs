use io::Write;
use std::{
    collections::{HashMap, VecDeque},
    io, str,
};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Orient {
    t: usize,
    b: usize,
    n: usize,
    s: usize,
    w: usize,
    e: usize,
}

impl Orient {
    fn roll(self, d: usize) -> Self {
        match d {
            0 => Self {
                t: self.s,
                b: self.n,
                n: self.t,
                s: self.b,
                w: self.w,
                e: self.e,
            },
            1 => Self {
                t: self.n,
                b: self.s,
                n: self.b,
                s: self.t,
                w: self.w,
                e: self.e,
            },
            2 => Self {
                t: self.e,
                b: self.w,
                n: self.n,
                s: self.s,
                w: self.t,
                e: self.b,
            },
            _ => Self {
                t: self.w,
                b: self.e,
                n: self.n,
                s: self.s,
                w: self.b,
                e: self.t,
            },
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut grid = [[0; 6]; 6];
    let mut location = [(0, 0); 7];

    for i in 0..6 {
        for j in 0..6 {
            grid[i][j] = scan.token::<usize>();

            if grid[i][j] != 0 {
                location[grid[i][j]] = (i, j);
            }
        }
    }

    let mut face: [Option<i64>; 6] = [None; 6];
    face[0] = Some(1);

    let mut queue = VecDeque::new();
    queue.push_back((
        location[1],
        Orient {
            t: 1,
            b: 0,
            n: 2,
            s: 3,
            w: 4,
            e: 5,
        },
    ));

    let mut visited = HashMap::new();
    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, -1, 1];

    while let Some(((r, c), ori)) = queue.pop_front() {
        if let Some(prev) = visited.get(&(r, c)) {
            if *prev != ori {
                writeln!(out, "0").unwrap();
                return;
            }

            continue;
        }

        visited.insert((r, c), ori);

        for d in 0..4 {
            let r_next = r as i64 + dr[d];
            let c_next = c as i64 + dc[d];

            if r_next < 0 || r_next >= 6 || c_next < 0 || c_next >= 6 {
                continue;
            }

            let (r_next, c_next) = (r_next as usize, c_next as usize);

            if grid[r_next][c_next] == 0 {
                continue;
            }

            let ori_next = ori.roll(d);

            match face[ori_next.b] {
                None => face[ori_next.b] = Some(grid[r_next][c_next] as i64),
                Some(x) if x == grid[r_next][c_next] as i64 => {}
                _ => {
                    writeln!(out, "0").unwrap();
                    return;
                }
            }

            queue.push_back(((r_next, c_next), ori_next));
        }
    }

    if face.iter().any(|&f| f.is_none()) {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{}", face[1].unwrap()).unwrap();
}
