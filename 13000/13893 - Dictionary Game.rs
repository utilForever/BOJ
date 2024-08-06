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

// Using Trie
fn add_word(
    children: &mut Vec<Vec<usize>>,
    parent: &mut Vec<usize>,
    num_node: &mut usize,
    word: &str,
) -> usize {
    let mut cur_idx = 0;

    for c in word.chars() {
        let ch = c as usize - 'a' as usize;

        if children[cur_idx][ch] == 0 {
            children[cur_idx][ch] = *num_node;
            parent[*num_node] = cur_idx;
            *num_node += 1;
        }

        cur_idx = children[cur_idx][ch];
    }

    cur_idx
}

fn process_dfs(children: &Vec<Vec<usize>>, grundy: &mut Vec<usize>, cur_idx: usize) {
    for i in 0..26 {
        if children[cur_idx][i] != 0 {
            process_dfs(children, grundy, children[cur_idx][i]);
        }
    }

    grundy[cur_idx] = 0;

    for i in 0..26 {
        if children[cur_idx][i] != 0 {
            grundy[cur_idx] ^= grundy[children[cur_idx][i]] + 1;
        }
    }
}

// Reference: https://rkm0959.tistory.com/139
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();
    let mut children = vec![vec![0; 26]; 50000 * 40];
    let mut parent = vec![0; 50000 * 40];
    let mut grundy = vec![0; 50000 * 40];

    for i in 0..t {
        children.iter_mut().for_each(|x| x.fill(0));
        parent.fill(0);
        grundy.fill(0);

        let n = scan.token::<usize>();
        let mut num_node = 1;

        for _ in 0..n {
            let dw = scan.token::<String>();
            add_word(&mut children, &mut parent, &mut num_node, &dw);
        }

        process_dfs(&children, &mut grundy, 0);
        writeln!(out, "Case {}:", i + 1).unwrap();

        let q = scan.token::<usize>();
        for _ in 0..q {
            let qw = scan.token::<String>();
            let mut cur_idx = add_word(&mut children, &mut parent, &mut num_node, &qw);

            loop {
                grundy[cur_idx] = 0;

                for j in 0..26 {
                    if children[cur_idx][j] != 0 {
                        grundy[cur_idx] ^= grundy[children[cur_idx][j]] + 1;
                    }
                }

                if cur_idx == 0 {
                    break;
                }

                cur_idx = parent[cur_idx];
            }

            writeln!(out, "{}", if grundy[0] == 0 { "2" } else { "1" }).unwrap();
        }
    }
}
