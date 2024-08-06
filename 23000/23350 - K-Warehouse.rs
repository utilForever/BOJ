use io::Write;
use std::{collections::VecDeque, io, str};

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

struct Container {
    priority: usize,
    weight: usize,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut containers = VecDeque::new();
    let mut priorities = vec![0; 100];

    for _ in 0..n {
        let (priority, weight) = (scan.token::<usize>(), scan.token::<usize>());
        containers.push_back(Container { priority, weight });
        priorities[priority] += 1;
    }

    let mut loaded_containers: Vec<Container> = Vec::new();
    let mut worked_count = 0;
    let mut sum_weight = 0;
    let mut cur_priority = m;

    while !containers.is_empty() {
        let container = containers.pop_front().unwrap();

        if container.priority == cur_priority {
            sum_weight += container.weight;

            for loaded_container in loaded_containers.iter() {
                if container.priority == loaded_container.priority
                    && container.weight > loaded_container.weight
                {
                    sum_weight += 2 * loaded_container.weight;
                }
            }

            loaded_containers.push(container);
            worked_count += 1;

            if worked_count == priorities[cur_priority] {
                cur_priority -= 1;
                worked_count = 0;
            }
        } else {
            sum_weight += container.weight;
            containers.push_back(container);
        }
    }

    writeln!(out, "{}", sum_weight).unwrap();
}
