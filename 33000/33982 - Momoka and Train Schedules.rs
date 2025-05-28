use io::Write;
use std::{
    collections::{HashMap, HashSet},
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

#[derive(Clone, Copy)]
struct Event {
    time: i64,
    station: usize,
    train: usize,
    is_last: bool,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut demands = vec![0; n];

    for i in 0..n {
        demands[i] = scan.token::<i64>();
    }

    let mut graph = HashMap::with_capacity(m);

    for _ in 0..m {
        let (mut u, mut v, w) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        graph.insert((u, v), w);
    }

    let mut paths = vec![Vec::new(); t];
    let mut events = Vec::with_capacity(500_000);
    let mut dup_time = vec![None; t];
    let mut edge_time = vec![None; t];
    let mut edge_stall = vec![None; t];

    for i in 0..t {
        let (k, l) = (scan.token::<usize>(), scan.token::<i64>());
        let mut routes = vec![0; k];

        for j in 0..k {
            routes[j] = scan.token::<usize>() - 1;
        }

        let mut visited = HashSet::with_capacity(k * 2);
        let mut time_curr = l;
        let mut station_curr = routes[0];

        events.push(Event {
            time: time_curr,
            station: station_curr,
            train: i,
            is_last: k == 1,
        });
        visited.insert(station_curr);

        let mut check = false;

        for j in 1..k {
            let station_next = routes[j];
            let key = if station_curr <= station_next {
                (station_curr, station_next)
            } else {
                (station_next, station_curr)
            };
            let time_next = if let Some(&time) = graph.get(&key) {
                time
            } else {
                edge_time[i] = Some(time_curr);
                edge_stall[i] = Some(station_curr);
                check = true;
                break;
            };

            time_curr += time_next;
            station_curr = station_next;

            if dup_time[i].is_none() && visited.contains(&station_next) {
                dup_time[i] = Some(time_curr);
            }

            events.push(Event {
                time: time_curr,
                station: station_curr,
                train: i,
                is_last: j + 1 == k,
            });
            visited.insert(station_next);
        }

        if !check {
            let last = events.len() - 1;
            events[last].is_last = true;
        }

        paths[i] = routes;
    }

    events.sort_by(|a, b| {
        if a.time == b.time {
            a.station.cmp(&b.station)
        } else {
            a.time.cmp(&b.time)
        }
    });

    let mut occupied = vec![false; n];
    let mut collision_time = vec![None; t];
    let mut live = vec![true; t];
    let mut idx = 0;

    while idx < events.len() {
        if !live[events[idx].train] {
            idx += 1;
            continue;
        }

        let time_curr = events[idx].time;
        let station_curr = events[idx].station;
        let mut idx2 = idx;

        while idx2 < events.len()
            && events[idx2].time == time_curr
            && events[idx2].station == station_curr
        {
            idx2 += 1;
        }

        let mut active = Vec::new();

        for event in events[idx..idx2].iter() {
            if live[event.train] {
                active.push(event);
            }
        }

        if active.is_empty() {
            idx = idx2;
            continue;
        }

        if occupied[station_curr] {
            for event in active.iter() {
                collision_time[event.train].get_or_insert(time_curr);
                live[event.train] = false;
            }
        } else if active.len() == 1 {
            if edge_time[active[0].train] == Some(time_curr) {
                occupied[station_curr] = true;
                live[active[0].train] = false;
            }
        } else {
            for event in active.iter() {
                collision_time[event.train].get_or_insert(time_curr);
                live[event.train] = false;
            }

            occupied[station_curr] = true;
        }

        idx = idx2;
    }

    let mut ret = Vec::with_capacity(t);
    let mut cnt = 0;

    for i in 0..t {
        let dt = dup_time[i].unwrap_or(i64::MAX);
        let et = edge_time[i].unwrap_or(i64::MAX);
        let ct = collision_time[i].unwrap_or(i64::MAX);
        let earliest = dt.min(et).min(ct);

        if earliest == i64::MAX {
            ret.push("YES");
            cnt += 1;
        } else if earliest == dt {
            ret.push("NO: Duplicate Visit");
        } else if earliest == ct {
            ret.push("NO: Collision");
        } else {
            ret.push("NO: Edge Absence");
        }
    }

    let mut pass = vec![0; n];

    for (idx, &status) in ret.iter().enumerate() {
        if status == "YES" {
            for &station in paths[idx].iter() {
                pass[station] += 1;
            }
        }
    }

    let filled = pass
        .iter()
        .zip(demands.iter())
        .all(|(have, need)| have >= need);

    writeln!(out, "{cnt}").unwrap();

    for status in ret {
        writeln!(out, "{status}").unwrap();
    }

    writeln!(out, "{}", if filled { "filled" } else { "unfilled" }).unwrap();
}
