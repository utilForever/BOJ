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

static INF_I64: i64 = 0x3f3f3f3f3f3f3f3f;

struct FunctionBox {
    h: usize,
    w: usize,
    n: usize,
    picture: Vec<Vec<char>>,
    graph_in: Vec<Vec<(i64, i64)>>,
    graph_out: Vec<Vec<(i64, i64)>>,
    s_actual: Vec<Vec<(i64, i64)>>,
    s_len: Vec<i64>,
    input: Vec<Vec<i64>>,
    output: Vec<Vec<i64>>,
    k: usize,
    a: Vec<usize>,
    b: Vec<usize>,
    c: Vec<usize>,
    d: Vec<Vec<i64>>,
    e: Vec<usize>,
    f: Vec<Vec<i64>>,
    s: Vec<String>,
    g: Vec<usize>,
}

impl FunctionBox {
    pub fn new(
        h: usize,
        w: usize,
        picture: Vec<Vec<char>>,
        k: usize,
        c: Vec<usize>,
        d: Vec<Vec<i64>>,
        e: Vec<usize>,
        f: Vec<Vec<i64>>,
        s: Vec<String>,
        g: Vec<usize>,
    ) -> Self {
        Self {
            h,
            w,
            n: 0,
            picture,
            graph_in: Vec::new(),
            graph_out: Vec::new(),
            s_actual: Vec::new(),
            s_len: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            k,
            a: Vec::new(),
            b: Vec::new(),
            c,
            d,
            e,
            f,
            s,
            g,
        }
    }

    fn output(&self, y: usize, o: usize) -> i64 {
        self.output[y][o]
    }

    fn get_char(&self, y: i64, mut x: i64) -> char {
        for &(id, val) in self.s_actual[y as usize].iter() {
            if id < 0 {
                if x == 0 {
                    return val as u8 as char;
                }

                x -= 1;
                continue;
            }

            if x < self.s_len[id as usize] {
                return self.get_char(id, x);
            }

            x -= self.s_len[id as usize];
        }

        unreachable!()
    }

    fn get_neighbours(
        &self,
        box_entries: &Vec<Vec<Vec<(usize, usize)>>>,
        i: usize,
        j: usize,
    ) -> ((usize, usize), (usize, usize)) {
        if self.picture[i][j] == '-' {
            ((i, j - 1), (i, j + 1))
        } else if self.picture[i][j] == '|' {
            ((i - 1, j), (i + 1, j))
        } else if self.picture[i][j] == '+' {
            (box_entries[i][j][0], box_entries[i][j][1])
        } else {
            ((0, 0), (0, 0))
        }
    }

    fn get_path_for_run(
        &self,
        box_entries: &Vec<Vec<Vec<(usize, usize)>>>,
        visited: &mut Vec<Vec<bool>>,
        mut curr: (usize, usize),
        mut prev: (usize, usize),
    ) -> (usize, usize) {
        while !visited[curr.0][curr.1] {
            let (mut a, mut b) = self.get_neighbours(&box_entries, curr.0, curr.1);

            if a == prev {
                std::mem::swap(&mut a, &mut b);
            }

            visited[curr.0][curr.1] = true;

            prev = curr;
            curr = a;
        }

        return curr;
    }

    fn parse(&mut self) {
        let dx: [i64; 4] = [1, 0, -1, 0];
        let dy: [i64; 4] = [0, 1, 0, -1];

        let mut box_entries = vec![vec![Vec::new(); self.w]; self.h];

        for i in 0..self.h {
            for j in 0..self.w {
                if self.picture[i][j] != '+' {
                    continue;
                }

                for k in 0..4 {
                    let next_i = i as i64 + dy[k];
                    let next_j = j as i64 + dx[k];

                    if next_i < 0
                        || next_i >= self.h as i64
                        || next_j < 0
                        || next_j >= self.w as i64
                    {
                        continue;
                    }

                    let next_i = next_i as usize;
                    let next_j = next_j as usize;
                    let ch = self.picture[next_i][next_j];
                    let is_horizontal = if k == 1 || k == 3 { true } else { false };
                    let is_vertical = if k == 0 || k == 2 { true } else { false };
                    let mut is_box = if ch == '+' { true } else { false };

                    if is_horizontal && (ch == '^' || ch == 'v' || ch == '|') {
                        is_box = true;
                    } else if is_vertical && (ch == '<' || ch == '>' || ch == '-') {
                        is_box = true;
                    }

                    if !is_box {
                        continue;
                    }

                    box_entries[i][j].push((next_i, next_j));
                }
            }
        }

        let mut arrows = Vec::new();

        for i in 0..self.h {
            for j in 0..self.w {
                if self.picture[i][j] == '^' || self.picture[i][j] == 'v' {
                    arrows.push(((i, j), self.picture[i][j]));
                    self.picture[i][j] = '-';
                } else if self.picture[i][j] == '<' || self.picture[i][j] == '>' {
                    arrows.push(((i, j), self.picture[i][j]));
                    self.picture[i][j] = '|';
                }
            }
        }

        let mut box_connectors = Vec::new();
        let mut visited = vec![vec![false; self.w]; self.h];

        for i in 0..self.h {
            for j in 0..self.w {
                if self.picture[i][j] != '+' {
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                let mut i_next = i + 1;
                let mut j_next = j + 1;

                while i_next < self.h && self.picture[i_next][j] == '|' {
                    i_next += 1;
                }

                while j_next < self.w && self.picture[i][j_next] == '-' {
                    j_next += 1;
                }

                if i_next < i + 2 || i_next >= self.h || j_next < j + 2 || j_next >= self.w {
                    continue;
                }

                let mut is_surrounded = self.picture[i][j_next] == '+'
                    && self.picture[i_next][j] == '+'
                    && self.picture[i_next][j_next] == '+';

                for y in i + 1..i_next {
                    is_surrounded &= self.picture[y][j_next] == '|';
                }

                for x in j + 1..j_next {
                    is_surrounded &= self.picture[i_next][x] == '-';
                }

                if !is_surrounded {
                    continue;
                }

                box_connectors.push(((i, j), (i_next, j_next)));

                for y in i..=i_next {
                    visited[y][j] = true;
                    visited[y][j_next] = true;
                }

                for x in j..=j_next {
                    visited[i][x] = true;
                    visited[i_next][x] = true;
                }
            }
        }

        for (pos, value) in arrows {
            self.picture[pos.0][pos.1] = value;
        }

        self.n = box_connectors.len();
        let mut boxes = vec![((0, 0), (0, 0)); self.n];

        for &((i_start, j_start), (i_end, j_end)) in box_connectors.iter() {
            let mut id = 0;

            for y in i_start + 1..i_end {
                for x in j_start + 1..j_end {
                    visited[y][x] = true;

                    if self.picture[y][x] == '.' {
                        continue;
                    }

                    id = id * 10 + (self.picture[y][x] as u8 & 15);
                }
            }

            boxes[id as usize] = ((i_start, j_start), (i_end, j_end));
        }

        let mut coordinates_in = vec![Vec::new(); self.n];
        let mut coordinates_out = vec![Vec::new(); self.n];

        for (idx, &((i_start, j_start), (i_end, j_end))) in boxes.iter().enumerate() {
            for x in j_start + 1..j_end {
                if self.picture[i_start][x] == 'v' {
                    coordinates_in[idx].push((i_start, x));
                } else if self.picture[i_start][x] == '^' {
                    coordinates_out[idx].push((i_start, x));
                }
            }

            for y in i_start + 1..i_end {
                if self.picture[y][j_end] == '<' {
                    coordinates_in[idx].push((y, j_end));
                } else if self.picture[y][j_end] == '>' {
                    coordinates_out[idx].push((y, j_end));
                }
            }

            for x in (j_start + 1..j_end).rev() {
                if self.picture[i_end][x] == '^' {
                    coordinates_in[idx].push((i_end, x));
                } else if self.picture[i_end][x] == 'v' {
                    coordinates_out[idx].push((i_end, x));
                }
            }

            for y in (i_start + 1..i_end).rev() {
                if self.picture[y][j_start] == '>' {
                    coordinates_in[idx].push((y, j_start));
                } else if self.picture[y][j_start] == '<' {
                    coordinates_out[idx].push((y, j_start));
                }
            }
        }

        self.a = vec![0; self.n];
        self.b = vec![0; self.n];

        for i in 0..self.n {
            self.a[i] = coordinates_in[i].len();
            self.b[i] = coordinates_out[i].len();
        }

        let mut map_input_output = vec![vec![(0, -1); self.w]; self.h];

        for i in 0..self.n {
            for j in 0..self.a[i] {
                let (y, x) = coordinates_in[i][j];
                map_input_output[y][x] = (i as i64 + 1, j as i64);
            }

            for j in 0..self.b[i] {
                let (y, x) = coordinates_out[i][j];
                map_input_output[y][x] = (-(i as i64 + 1), j as i64);
            }
        }

        self.graph_in = vec![Vec::new(); self.n];
        self.graph_out = vec![Vec::new(); self.n];

        for i in 0..self.n {
            self.graph_in[i] = vec![(-1, -1); self.a[i]];
            self.graph_out[i] = vec![(-1, -1); self.b[i]];
        }

        for i in 0..self.h {
            for j in 0..self.w {
                if visited[i][j] {
                    continue;
                }

                if self.picture[i][j] == '.' {
                    continue;
                }

                visited[i][j] = true;

                let (a, b) = self.get_neighbours(&box_entries, i, j);
                let e1 = self.get_path_for_run(&box_entries, &mut visited, a, (i, j));
                let e2 = self.get_path_for_run(&box_entries, &mut visited, b, (i, j));
                let mut d1 = map_input_output[e1.0][e1.1];
                let mut d2 = map_input_output[e2.0][e2.1];

                if d1.0 > 0 {
                    std::mem::swap(&mut d1, &mut d2);
                }

                self.graph_in[(d2.0 - 1) as usize][d2.1 as usize] = (-d1.0 - 1, d1.1);
                self.graph_out[(-d1.0 - 1) as usize][d1.1 as usize] = (d2.0 - 1, d2.1);
            }
        }

        for i in 0..self.n {
            for j in 0..self.b[i] {
                let (y, x) = coordinates_out[i][j];
                let (mut y_next, mut x_next) = (-1, -1);

                if self.picture[y][x] == 'v' {
                    y_next = (y + 1) as i64;
                    x_next = x as i64;
                } else if self.picture[y][x] == '>' {
                    y_next = y as i64;
                    x_next = (x + 1) as i64;
                } else if self.picture[y][x] == '^' {
                    y_next = (y - 1) as i64;
                    x_next = x as i64;
                } else if self.picture[y][x] == '<' {
                    y_next = y as i64;
                    x_next = (x - 1) as i64;
                }

                if y_next < 0 || y_next >= self.h as i64 || x_next < 0 || x_next >= self.w as i64 {
                    continue;
                }

                let d = map_input_output[y_next as usize][x_next as usize];

                if d.0 == 0 {
                    continue;
                }

                self.graph_in[d.0 as usize - 1][d.1 as usize] = (i as i64, j as i64);
                self.graph_out[i][j] = (d.0 - 1, d.1);
            }
        }
    }

    fn process_strings(&mut self) {
        self.s_actual = vec![Vec::new(); self.k];
        self.s_len = vec![0; self.k];

        for i in 0..self.k {
            let s = self.s[i].chars().collect::<Vec<_>>();
            let len = s.len();
            let mut idx = 0;

            while idx < len {
                if s[idx] != '(' {
                    self.s_actual[i].push((-1, s[idx] as i64));
                    idx += 1;
                    continue;
                }

                idx += 1;

                let mut id = 0;

                while idx < len {
                    if s[idx] < '0' || s[idx] > '9' {
                        break;
                    }

                    id = id * 10 + (s[idx] as u8 & 15);
                    idx += 1;
                }

                idx += 1;

                self.s_actual[i].push((id as i64, 0));
            }

            let mut len_total = 0;

            for &(id, _) in self.s_actual[i].iter() {
                len_total += if id == -1 {
                    1
                } else {
                    self.s_len[id as usize]
                };

                if len_total > INF_I64 {
                    len_total = INF_I64;
                }
            }

            self.s_len[i] = len_total;
        }

        self.input = vec![Vec::new(); self.n];
        self.output = vec![Vec::new(); self.n];

        let mut proc = vec![0; self.n];

        for i in 0..self.n {
            self.input[i] = vec![0; self.a[i]];
            self.output[i] = vec![0; self.b[i]];
        }

        for i in 0..self.b[0] {
            self.output[0][i] = self.s_len[self.g[i]];
        }

        for i in 1..self.n {
            for j in 0..self.a[i] {
                let d = self.graph_in[i][j];

                self.input[i][j] = if d.0 < 0 {
                    0
                } else {
                    self.output[d.0 as usize][d.1 as usize]
                };
            }

            let mut visited = vec![false; self.a[i]];
            let mut total = 0;

            for j in 0..self.c[i] {
                visited[self.d[i][j] as usize] = true;
            }


            for j in 0..self.a[i] {
                if visited[j] {
                    total += self.input[i][j];
                }
            }

            proc[i] = total;

            let total = proc[i] as usize;
            let q = total / self.e[i];
            let r = total % self.e[i];

            for j in 0..self.e[i] {
                let val = self.f[i][j];

                if j < r {
                    self.output[i][val as usize] += 1;
                }

                self.output[i][val as usize] += q as i64;
            }
        }
    }

    fn solve(&self, y: usize, o: usize, mut p: i64) -> char {
        if y == 0 {
            return self.get_char(self.g[o] as i64, p as i64);
        }

        let mut idxes_out = Vec::new();

        for i in 0..self.e[y] {
            if self.f[y][i] == o as i64 {
                idxes_out.push(i as i64);
            }
        }

        let q = p / idxes_out.len() as i64;
        let r = p % idxes_out.len() as i64;

        p = q * self.e[y] as i64 + idxes_out[r as usize] + 1;

        let mut idxes_proc = Vec::new();
        let mut visited = vec![false; self.a[y]];

        for i in 0..self.c[y] {
            let idx = self.d[y][i] as usize;

            if visited[idx] {
                continue;
            }

            idxes_proc.push(idx);
            visited[idx] = true;
        }

        let mut checked = vec![true; self.c[y]];
        let mut used = vec![0; self.a[y]];
        let mut cnt = vec![0; self.a[y]];

        while p != 0 {
            for &idx in idxes_proc.iter() {
                cnt[idx] = 0;
            }

            let mut len_pattern = 0;

            for i in 0..self.c[y] {
                if !checked[i] {
                    continue;
                }

                let idx = self.d[y][i] as usize;

                if self.input[y][idx] < used[idx] + cnt[idx] + 1 {
                    checked[i] = false;
                    continue;
                }

                cnt[idx] += 1;
                len_pattern += 1;
            }

            let mut total = INF_I64;

            for &idx in idxes_proc.iter() {
                if cnt[idx] > 0 {
                    total = total.min((self.input[y][idx] - used[idx]) / cnt[idx]);
                }
            }

            if total * len_pattern < p {
                p -= total * len_pattern;

                for &idx in idxes_proc.iter() {
                    used[idx] += total * cnt[idx];
                }

                continue;
            }

            let mut q = p / len_pattern;
            let mut r = p % len_pattern;

            if r == 0 {
                q -= 1;
                r = len_pattern;
            }

            for &idx in idxes_proc.iter() {
                used[idx] += q * cnt[idx];
            }

            let mut last = -1;
            let mut i = 0;
            let mut j = 0;

            while i < self.c[y] && j < r {
                if !checked[i] {
                    i += 1;
                    continue;
                }

                let idx = self.d[y][i] as usize;
                last = idx as i64;

                used[idx] += 1;
                i += 1;
                j += 1;
            }

            let d = self.graph_in[y][last as usize];

            return self.solve(d.0 as usize, d.1 as usize, used[last as usize] - 1);
        }

        unreachable!()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    // Input
    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut picture = vec![vec![' '; w]; h];

    for i in 0..h {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            picture[i][j] = c;
        }
    }

    let n_prime = scan.token::<usize>();
    let mut c = vec![0; n_prime];
    let mut d = vec![Vec::new(); n_prime];
    let mut e = vec![0; n_prime];
    let mut f = vec![Vec::new(); n_prime];

    for i in 0..n_prime {
        c[i] = scan.token::<usize>();
        d[i] = vec![0; c[i]];

        for j in 0..c[i] {
            d[i][j] = scan.token::<i64>();
        }
    }

    for i in 0..n_prime {
        e[i] = scan.token::<usize>();
        f[i] = vec![0; e[i]];

        for j in 0..e[i] {
            f[i][j] = scan.token::<i64>();
        }
    }

    let k = scan.token::<usize>();
    let mut s = vec![String::new(); k];

    for i in 0..k {
        s[i] = scan.token::<String>();
    }

    let m_prime = scan.token::<usize>();
    let mut g = vec![0; m_prime];

    for i in 0..m_prime {
        g[i] = scan.token::<usize>();
    }

    let q = scan.token::<usize>();
    let mut y = vec![0; q];
    let mut o = vec![0; q];
    let mut p = vec![0; q];

    for i in 0..q {
        y[i] = scan.token::<usize>();
    }

    for i in 0..q {
        o[i] = scan.token::<usize>();
    }

    for i in 0..q {
        p[i] = scan.token::<i64>();
    }

    let mut function_box = FunctionBox::new(h, w, picture, k, c, d, e, f, s, g);

    function_box.parse();
    function_box.process_strings();

    for i in 0..q {
        if function_box.output(y[i], o[i]) < p[i] {
            write!(out, "!").unwrap();
        } else {
            write!(out, "{}", function_box.solve(y[i], o[i], p[i] - 1)).unwrap();
        }
    }

    writeln!(out).unwrap();
}
