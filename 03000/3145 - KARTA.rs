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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    matched: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched[next] == -1 || process_dfs(graph, check, matched, matched[next] as usize) {
            matched[next] = idx as i64;
            return true;
        }
    }

    false
}

struct Name {
    name: String,
    letters: Vec<(usize, usize)>,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![' '; c]; r];

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            map[i][j] = c;
        }
    }

    let mut towns = Vec::new();
    let mut names = Vec::new();

    for i in 0..r {
        for j in 0..c {
            if map[i][j] == 'x' {
                towns.push((i, j));
            }
        }
    }

    for i in 0..r {
        let mut j = 0;

        while j < c {
            if map[i][j].is_ascii_uppercase() {
                let mut col_curr = j;

                while col_curr < c && map[i][col_curr].is_ascii_uppercase() {
                    col_curr += 1;
                }

                let name = map[i][j..col_curr].iter().collect::<String>();
                let mut letters = Vec::new();

                for k in j..col_curr {
                    letters.push((i, k));
                }

                names.push(Name { name, letters });

                j = col_curr;
            } else {
                j += 1;
            }
        }
    }

    let cnt_towns = towns.len();
    let cnt_names = names.len();
    let mut graph = vec![Vec::new(); cnt_towns];

    for (i, &(town_y, town_x)) in towns.iter().enumerate() {
        for (j, name) in names.iter().enumerate() {
            let mut can_match = false;

            for &(r, c) in name.letters.iter() {
                if (r as i64 - town_y as i64).abs() <= 1 && (c as i64 - town_x as i64).abs() <= 1 {
                    can_match = true;
                    break;
                }
            }

            if can_match {
                graph[i].push(j);
            }
        }
    }

    let mut check = vec![false; cnt_names];
    let mut matched = vec![-1; cnt_names];

    for i in 0..cnt_towns {
        check.fill(false);

        if process_dfs(&graph, &mut check, &mut matched, i) {
            // Do nothing
        }
    }

    for (idx_name, &idx_town) in matched.iter().enumerate() {
        if idx_town == -1 {
            continue;
        }

        let (town_y, town_x) = towns[idx_town as usize];

        writeln!(
            out,
            "{} {} {}",
            town_y + 1,
            town_x + 1,
            names[idx_name].name
        )
        .unwrap();
    }
}
