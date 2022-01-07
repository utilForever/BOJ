use std::{
    cmp,
    collections::BinaryHeap,
    hint::unreachable_unchecked,
    io::{stdin, Read},
    str,
};

#[derive(Clone, Default, Eq, PartialEq, Ord)]
struct Partial {
    sum_start: u32,
    time: u32,
    num_photo: u32,
    cur_idx: u16,
    deadline_idx: u16,
    after_photo: bool,
}

impl PartialOrd for Partial {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.time != other.time {
            return if self.time > other.time {
                Some(cmp::Ordering::Less)
            } else {
                Some(cmp::Ordering::Greater)
            };
        }

        if self.num_photo != other.num_photo {
            return if self.num_photo < other.num_photo {
                Some(cmp::Ordering::Less)
            } else {
                Some(cmp::Ordering::Greater)
            };
        }

        if self.sum_start < other.sum_start {
            Some(cmp::Ordering::Less)
        } else {
            Some(cmp::Ordering::Greater)
        }
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let input = buffer.split_ascii_whitespace();
    let mut nums = input.map(str::parse::<u32>).flatten();

    let (n, t): (u32, u32) = (nums.by_ref().next().unwrap(), nums.by_ref().next().unwrap());
    let mut working_times: Vec<(i32, i32)> = vec![(0, 0); n as usize];

    for time in working_times.iter_mut() {
        *time = (
            nums.by_ref().next().unwrap() as i32,
            nums.by_ref().next().unwrap() as i32,
        );
    }

    working_times.sort_unstable();

    let mut ret = false;
    let mut pq: BinaryHeap<Partial> = BinaryHeap::new();
    let mut deadlines: Vec<BinaryHeap<i32>> = Vec::new();
    deadlines.push(BinaryHeap::new());

    let start = Partial::default();

    pq.push(start);

    while !pq.is_empty() {
        let mut p = pq
            .peek()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
            .clone();

        while !pq.is_empty()
            && pq
                .peek()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
                .time
                == p.time
        {
            pq.pop();
        }

        if p.num_photo == n {
            ret = true;
            break;
        }

        if p.after_photo {
            unsafe { deadlines.get_unchecked_mut(p.deadline_idx as usize).pop() };
        } else {
            unsafe { deadlines.push(deadlines.get_unchecked(p.deadline_idx as usize).clone()) };
            p.deadline_idx = deadlines.len() as u16 - 1;
        }

        let deadline = unsafe { &mut deadlines.get_unchecked_mut(p.deadline_idx as usize) };

        loop {
            if p.cur_idx >= working_times.len() as u16 {
                break;
            }

            let working_time = unsafe { working_times.get_unchecked(p.cur_idx as usize) };

            if working_time.0 > p.time as i32 {
                break;
            }

            deadline.push(-working_time.1);

            p.cur_idx += 1;
        }

        if !deadline.is_empty()
            && -deadline
                .peek()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
                < (p.time + t) as i32
        {
            continue;
        }

        if p.cur_idx < working_times.len() as u16 {
            let start_time = unsafe { working_times.get_unchecked(p.cur_idx as usize).0 as u32 };

            if deadline.is_empty() || start_time < p.time + t {
                p.after_photo = false;

                let tmp = p.time;
                p.time = start_time;
                pq.push(p.clone());
                p.time = tmp;
            }
        }

        if !deadline.is_empty() {
            p.after_photo = true;

            p.num_photo += 1;
            p.sum_start += p.time;
            p.time += t;
            pq.push(p.clone());
        }
    }

    println!("{}", if ret { "yes" } else { "no" });
}
