use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let tc = input_integers()[0];

    for _ in 0..tc {
        let nums = input_integers();
        let (n, m, w) = (nums[0] as usize, nums[1] as usize, nums[2] as usize);

        let mut vertex_info = vec![Vec::new(); n + 1];

        for _ in 0..m {
            let nums = input_integers();
            let (s, e, t) = (nums[0], nums[1], nums[2]);

            vertex_info[s as usize].push((e, t));
            vertex_info[e as usize].push((s, t));
        }

        for _ in 0..w {
            let nums = input_integers();
            let (s, e, t) = (nums[0], nums[1], nums[2]);

            vertex_info[s as usize].push((e, -t));
        }

        let mut times = vec![0; n + 1];
        let mut has_cycle = false;

        for i in 1..=n {
            for j in 1..=n {
                for (to_vertex, time) in vertex_info[j].iter() {
                    if times[*to_vertex as usize] > times[j] + time {
                        times[*to_vertex as usize] = times[j] + time;

                        if i == n {
                            has_cycle = true;
                        }
                    }
                }
            }
        }

        println!("{}", if has_cycle { "YES" } else { "NO" });
    }
}
