use io::Write;
use std::{
    collections::{BTreeMap, HashMap},
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

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut heights: HashMap<usize, i64> = HashMap::new();
    let mut cols: Vec<BTreeMap<i64, i64>> = vec![BTreeMap::new(); n];
    let mut rows: Vec<BTreeMap<i64, i64>> = vec![BTreeMap::new(); m];
    let mut max_col = vec![0; n];
    let mut max_row = vec![0; m];
    let mut ret_front = 0;
    let mut ret_side = 0;
    let mut ret_top = 0;

    for _ in 0..q {
        let cmd = scan.token::<String>();

        match cmd.as_str() {
            "STACK" => {
                let (i, j) = (scan.token::<usize>(), scan.token::<usize>());
                let pos = i * m + j;

                let val_old = *heights.get(&pos).unwrap_or(&0);
                let val_new = val_old + 1;

                heights.insert(pos, val_new);

                // Update column i
                {
                    if val_old > 0 {
                        let entry = cols[i].get_mut(&val_old).unwrap();
                        *entry -= 1;

                        if *entry == 0 {
                            cols[i].remove(&val_old);
                        }
                    }

                    *cols[i].entry(val_new).or_insert(0) += 1;

                    if val_new > max_col[i] {
                        ret_front += val_new - max_col[i];
                        max_col[i] = val_new;
                    }
                }

                // Update row j
                {
                    if val_old > 0 {
                        let entry = rows[j].get_mut(&val_old).unwrap();
                        *entry -= 1;

                        if *entry == 0 {
                            rows[j].remove(&val_old);
                        }
                    }

                    *rows[j].entry(val_new).or_insert(0) += 1;

                    if val_new > max_row[j] {
                        ret_side += val_new - max_row[j];
                        max_row[j] = val_new;
                    }
                }

                if val_old == 0 {
                    ret_top += 1;
                }
            }
            "REMOVE" => {
                let (i, j) = (scan.token::<usize>(), scan.token::<usize>());
                let pos = i * m + j;
                let val_old = match heights.get(&pos) {
                    Some(&val) => val,
                    _ => continue,
                };
                let val_new = val_old - 1;

                if val_new == 0 {
                    heights.remove(&pos);
                } else {
                    heights.insert(pos, val_new);
                }

                // Update column i
                {
                    let entry = cols[i].get_mut(&val_old).unwrap();
                    *entry -= 1;

                    if *entry == 0 {
                        cols[i].remove(&val_old);
                    }

                    if val_new > 0 {
                        *cols[i].entry(val_new).or_insert(0) += 1;
                    }

                    if val_old == max_col[i] && !cols[i].contains_key(&val_old) {
                        let max_new = cols[i].iter().next_back().map(|(&k, _)| k).unwrap_or(0);

                        ret_front -= val_old - max_new;
                        max_col[i] = max_new;
                    }
                }

                // Update row j
                {
                    let entry = rows[j].get_mut(&val_old).unwrap();
                    *entry -= 1;

                    if *entry == 0 {
                        rows[j].remove(&val_old);
                    }

                    if val_new > 0 {
                        *rows[j].entry(val_new).or_insert(0) += 1;
                    }

                    if val_old == max_row[j] && !rows[j].contains_key(&val_old) {
                        let max_new = rows[j].iter().next_back().map(|(&k, _)| k).unwrap_or(0);

                        ret_side -= val_old - max_new;
                        max_row[j] = max_new;
                    }
                }

                if val_new == 0 {
                    ret_top -= 1;
                }
            }
            "FRONT" => {
                writeln!(out, "{ret_front}").unwrap();
            }
            "SIDE" => {
                writeln!(out, "{ret_side}").unwrap();
            }
            "TOP" => {
                writeln!(out, "{ret_top}").unwrap();
            }
            _ => unreachable!(),
        }
    }
}
