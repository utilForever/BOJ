use io::Write;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
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

#[derive(Clone)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    edge_left: i64,
    edge_right: i64,
    time_last_updated: i64,
    weight: i64,
    dir: i8,
    anchored_left: bool,
    anchored_right: bool,
    alive: bool,
}

impl Node {
    fn len2(&self) -> i64 {
        self.edge_right - self.edge_left
    }

    fn update_to(&mut self, t2: i64) {
        if !self.alive {
            return;
        }

        if self.dir == 0 {
            self.time_last_updated = t2;
            return;
        }

        let dt = t2 - self.time_last_updated;

        if dt != 0 {
            let d = (self.dir as i64) * dt;

            self.edge_left += d;
            self.edge_right += d;
            self.time_last_updated = t2;
        }
    }

    fn touch_left_wall(&mut self) {
        let len2 = self.len2();

        self.edge_left = 0;
        self.edge_right = len2;
        self.dir = 0;
        self.anchored_left = true;
    }

    fn touch_right_wall(&mut self, n2: i64) {
        let len2 = self.len2();

        self.edge_left = n2 - len2;
        self.edge_right = n2;
        self.dir = 0;
        self.anchored_right = true;
    }
}

#[derive(Clone)]
struct Gap {
    left: usize,
    right: usize,
    ver: u64,
    time_event: Option<i64>,
}

#[derive(Eq, PartialEq)]
struct Event {
    time: i64,
    left: usize,
    ver: u64,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.time.cmp(&other.time) {
            Ordering::Equal => match self.left.cmp(&other.left) {
                Ordering::Equal => self.ver.cmp(&other.ver),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Simulator {
    n: i64,
    nodes: Vec<Node>,
    gaps: Vec<Option<Gap>>,
    heap: BinaryHeap<Reverse<Event>>,
    last_time: i64,
    wall_left: usize,
    wall_right: usize,
}

impl Simulator {
    fn new(mut items: Vec<(i64, i64, i8)>, n: i64) -> Self {
        items.sort_by_key(|&(x, _, _)| x);

        let m = items.len();
        let wall_left = 0;
        let wall_right = m + 1;
        let mut nodes = Vec::with_capacity(m + 2);

        nodes.push(Node {
            prev: None,
            next: if m > 0 { Some(1) } else { Some(wall_right) },
            edge_left: 0,
            edge_right: 0,
            time_last_updated: 0,
            weight: i64::MAX / 4,
            dir: 0,
            anchored_left: true,
            anchored_right: true,
            alive: true,
        });

        for (i, &(x, w, d)) in items.iter().enumerate() {
            let left = 2 * (x - 1);
            let right = 2 * x;

            nodes.push(Node {
                prev: if i == 0 { Some(0) } else { Some(i) },
                next: if i + 1 == m {
                    Some(wall_right)
                } else {
                    Some(i + 2)
                },
                edge_left: left,
                edge_right: right,
                time_last_updated: 0,
                weight: w,
                dir: d,
                anchored_left: false,
                anchored_right: false,
                alive: true,
            });
        }

        nodes.push(Node {
            prev: if m > 0 { Some(m) } else { Some(0) },
            next: None,
            edge_left: n * 2,
            edge_right: n * 2,
            time_last_updated: 0,
            weight: i64::MAX / 4,
            dir: 0,
            anchored_left: true,
            anchored_right: true,
            alive: true,
        });

        if m == 0 {
            nodes[0].next = Some(wall_right);
            nodes[wall_right].prev = Some(0);
        } else {
            nodes[0].next = Some(1);
            nodes[wall_right].prev = Some(m);
        }

        let mut simulator = Simulator {
            n: n * 2,
            nodes,
            gaps: vec![None; m + 2],
            heap: BinaryHeap::new(),
            last_time: 0,
            wall_left,
            wall_right,
        };

        for left in 0..=m {
            if let Some(right) = simulator.nodes[left].next {
                simulator.setup_gap(left, right);
            }
        }

        simulator
    }

    fn calc_event_time(&self, left: usize, right: usize) -> Option<i64> {
        let a = &self.nodes[left];
        let b = &self.nodes[right];

        if !a.alive || !b.alive {
            return None;
        }

        let (dir_left, dir_right) = (a.dir, b.dir);

        if left == self.wall_left {
            if dir_right == -1 {
                return Some(b.edge_left + b.time_last_updated);
            } else {
                return None;
            }
        }

        if right == self.wall_right {
            if dir_left == 1 {
                return Some(self.n - a.edge_right + a.time_last_updated);
            } else {
                return None;
            }
        }

        match (dir_left, dir_right) {
            (1, -1) => {
                let val = b.edge_left - a.edge_right + a.time_last_updated + b.time_last_updated;

                if val % 2 != 0 {
                    return None;
                }

                let time = val / 2;

                if time >= a.time_last_updated && time >= b.time_last_updated {
                    Some(time)
                } else {
                    None
                }
            }
            (1, 0) => {
                let time = (b.edge_left - a.edge_right) + a.time_last_updated;

                if time >= a.time_last_updated && time >= b.time_last_updated {
                    Some(time)
                } else {
                    None
                }
            }
            (0, -1) => {
                let time = (b.edge_left - a.edge_right) + b.time_last_updated;

                if time >= a.time_last_updated && time >= b.time_last_updated {
                    Some(time)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn setup_gap(&mut self, left: usize, right: usize) {
        let time = self.calc_event_time(left, right);
        let entry = self.gaps[left].get_or_insert(Gap {
            left,
            right,
            ver: 0,
            time_event: None,
        });

        entry.right = right;
        entry.ver += 1;
        entry.time_event = time;

        if let Some(val) = time {
            self.heap.push(Reverse(Event {
                time: val,
                left,
                ver: entry.ver,
            }));
        }
    }

    fn invalidate_gap(&mut self, left: usize) {
        if let Some(ref mut gap) = self.gaps[left] {
            gap.ver += 1;
            gap.time_event = None;
        }
    }

    fn simulate(&mut self) -> (i64, usize) {
        while let Some(Reverse(event)) = self.heap.pop() {
            let Some(gap) = self.gaps.get(event.left).and_then(|x| x.as_ref()) else {
                continue;
            };

            if gap.ver != event.ver {
                continue;
            }

            let left = gap.left;
            let right = gap.right;

            if self.nodes[left].next != Some(right) {
                continue;
            }

            if let Some(event_time) = self.calc_event_time(left, right) {
                if event_time != event.time {
                    continue;
                }
            } else {
                continue;
            }

            self.last_time = event.time;

            let dir_left = self.nodes[left].dir;
            let dir_right = self.nodes[right].dir;

            if left == self.wall_left {
                let r = right;

                self.nodes[r].update_to(event.time);
                self.nodes[r].touch_left_wall();

                self.invalidate_gap(left);

                if let Some(node_right) = self.nodes[r].next {
                    self.setup_gap(r, node_right);
                }

                continue;
            }

            if right == self.wall_right {
                let l = left;

                self.nodes[l].update_to(event.time);
                self.nodes[l].touch_right_wall(self.n);

                self.invalidate_gap(l);

                if let Some(prev_left) = self.nodes[l].prev {
                    self.setup_gap(prev_left, l);
                }

                continue;
            }

            let mut handled_triple = false;

            if (dir_left == 1 && dir_right == 0) || (dir_left == 0 && dir_right == -1) {
                let center = if dir_right == 0 { right } else { left };
                let neighbor_left = self.nodes[center].prev.unwrap();
                let neighbor_right = self.nodes[center].next.unwrap();
                let time_left = self.calc_event_time(neighbor_left, center);
                let time_right = self.calc_event_time(center, neighbor_right);

                if time_left.is_some()
                    && time_right.is_some()
                    && time_left.unwrap() == event.time
                    && time_right.unwrap() == event.time
                {
                    handled_triple = true;

                    let idx_left = neighbor_left;
                    let idx_center = center;
                    let idx_right = neighbor_right;

                    self.nodes[idx_left].update_to(event.time);
                    self.nodes[idx_center].update_to(event.time);
                    self.nodes[idx_right].update_to(event.time);

                    let len_left = self.nodes[idx_left].len2();
                    let len_right = self.nodes[idx_right].len2();

                    let len_left_new = self.nodes[idx_center].edge_left - len_left;
                    let len_right_new = self.nodes[idx_center].edge_right + len_right;

                    let weight_left = self.nodes[idx_left].weight;
                    let weight_center = self.nodes[idx_center].weight;
                    let weight_right = self.nodes[idx_right].weight;

                    let anchored = self.nodes[idx_center].anchored_left
                        || self.nodes[idx_center].anchored_right;
                    let dir_new = if anchored {
                        0
                    } else if weight_left == weight_right
                        || (weight_left <= weight_center && weight_right <= weight_center)
                    {
                        0
                    } else {
                        if weight_left > weight_right {
                            1
                        } else {
                            -1
                        }
                    };

                    let outer_left = self.nodes[idx_left].prev.unwrap();
                    let outer_right = self.nodes[idx_right].next.unwrap();

                    self.invalidate_gap(outer_left);
                    self.invalidate_gap(idx_left);
                    self.invalidate_gap(idx_center);
                    self.invalidate_gap(idx_right);

                    self.nodes[idx_left].alive = false;
                    self.nodes[idx_right].alive = false;

                    let node_center = &mut self.nodes[idx_center];
                    node_center.edge_left = len_left_new;
                    node_center.edge_right = len_right_new;
                    node_center.time_last_updated = event.time;
                    node_center.weight = weight_left + weight_center + weight_right;
                    node_center.dir = dir_new;

                    self.nodes[outer_left].next = Some(idx_center);
                    self.nodes[idx_center].prev = Some(outer_left);
                    self.nodes[idx_center].next = Some(outer_right);
                    self.nodes[outer_right].prev = Some(idx_center);

                    self.setup_gap(outer_left, idx_center);
                    self.setup_gap(idx_center, outer_right);
                }
            }

            if handled_triple {
                continue;
            }

            match (dir_left, dir_right) {
                (1, -1) => {
                    let l = left;
                    let r = right;

                    self.nodes[l].update_to(event.time);
                    self.nodes[r].update_to(event.time);

                    let weight_left = self.nodes[l].weight;
                    let weight_right = self.nodes[r].weight;

                    let left_new = self.nodes[l].edge_left;
                    let right_new = self.nodes[r].edge_right;
                    let weight_new = weight_left + weight_right;
                    let dir_new = if weight_left > weight_right {
                        1
                    } else if weight_left < weight_right {
                        -1
                    } else {
                        0
                    };

                    let left_prev = self.nodes[l].prev.unwrap();
                    let right_next = self.nodes[r].next.unwrap();

                    self.invalidate_gap(left_prev);
                    self.invalidate_gap(l);
                    self.invalidate_gap(r);

                    self.nodes[r].alive = false;

                    let node_left = &mut self.nodes[l];
                    node_left.edge_left = left_new;
                    node_left.edge_right = right_new;
                    node_left.time_last_updated = event.time;
                    node_left.weight = weight_new;
                    node_left.dir = dir_new;
                    node_left.anchored_left =
                        node_left.anchored_left || (node_left.edge_left == 0 && dir_new == 0);
                    node_left.anchored_right = node_left.anchored_right
                        || (node_left.edge_right == self.n && dir_new == 0);

                    self.nodes[left_prev].next = Some(l);
                    self.nodes[l].prev = Some(left_prev);
                    self.nodes[l].next = Some(right_next);
                    self.nodes[right_next].prev = Some(l);

                    self.setup_gap(left_prev, l);
                    self.setup_gap(l, right_next);
                }
                (1, 0) | (0, -1) => {
                    let (id_moving_left, id_center, from_left) = if dir_right == 0 {
                        (left, right, true)
                    } else {
                        (right, left, false)
                    };
                    let moving = id_moving_left;
                    let center = id_center;

                    self.nodes[moving].update_to(event.time);
                    self.nodes[center].update_to(event.time);

                    let weight_moving = self.nodes[moving].weight;
                    let weight_center = self.nodes[center].weight;
                    let anchored =
                        self.nodes[center].anchored_left || self.nodes[center].anchored_right;
                    let dir_new = if anchored {
                        0
                    } else {
                        if weight_moving > weight_center {
                            self.nodes[moving].dir
                        } else {
                            0
                        }
                    };

                    let (left_new, right_new) = if from_left {
                        (self.nodes[moving].edge_left, self.nodes[center].edge_right)
                    } else {
                        (self.nodes[center].edge_left, self.nodes[moving].edge_right)
                    };

                    let left_prev = if from_left {
                        self.nodes[moving].prev.unwrap()
                    } else {
                        self.nodes[center].prev.unwrap()
                    };
                    let right_next = if from_left {
                        self.nodes[center].next.unwrap()
                    } else {
                        self.nodes[moving].next.unwrap()
                    };

                    self.invalidate_gap(left_prev);
                    if from_left {
                        self.invalidate_gap(moving);
                        self.invalidate_gap(center);
                    } else {
                        self.invalidate_gap(center);
                        self.invalidate_gap(moving);
                    }

                    self.nodes[moving].alive = false;

                    let node_center = &mut self.nodes[center];
                    node_center.edge_left = left_new;
                    node_center.edge_right = right_new;
                    node_center.time_last_updated = event.time;
                    node_center.weight = weight_moving + weight_center;
                    node_center.dir = dir_new;

                    self.nodes[left_prev].next = Some(center);
                    self.nodes[center].prev = Some(left_prev);
                    self.nodes[center].next = Some(right_next);
                    self.nodes[right_next].prev = Some(center);

                    self.setup_gap(left_prev, center);
                    self.setup_gap(center, right_next);
                }
                _ => {
                    // Do nothing
                }
            }
        }

        let mut cnt = 0;

        for i in 1..self.nodes.len() - 1 {
            if self.nodes[i].alive {
                cnt += 1;
            }
        }

        (self.last_time, cnt)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut items = vec![(0, 0, 0); m];

    for i in 0..m {
        let (x, w, d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<char>(),
        );
        items[i] = (x, w, if d == 'L' { -1 } else { 1 });
    }

    let mut simulator = Simulator::new(items, n);
    let (ret_time, ret_cnt) = simulator.simulate();

    if ret_time % 2 == 0 {
        writeln!(out, "{}.0 {}", ret_time / 2, ret_cnt).unwrap();
    } else {
        writeln!(out, "{}.5 {}", ret_time / 2, ret_cnt).unwrap();
    }
}
