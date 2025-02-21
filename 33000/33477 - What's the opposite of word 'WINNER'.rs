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

fn find(
    scan: &mut UnsafeScanner<io::StdinLock<'_>>,
    min: &mut Vec<i64>,
    max: &mut Vec<i64>,
    mut idx: usize,
    n: usize,
) {
    if idx > n {
        idx -= n;
    }

    if min[idx] == 0 {
        println!("? {idx}");
        (min[idx], max[idx]) = (scan.token::<i64>(), scan.token::<i64>());

        min[idx + n] = min[idx];
        max[idx + n] = max[idx];
    }
}

fn check(
    scan: &mut UnsafeScanner<io::StdinLock<'_>>,
    min: &mut Vec<i64>,
    max: &mut Vec<i64>,
    a1: &mut i64,
    a2: &mut i64,
    idx: usize,
    n: usize,
) -> bool {
    if *a1 == 0 && min[idx] == 1 {
        let mut left = idx;
        let mut right = idx + 99;

        while left <= right {
            let mid = (left + right) / 2;

            find(scan, min, max, mid, n);

            if min[mid] == 1 {
                *a1 = mid as i64;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    if *a2 == 0 && max[idx] == n as i64 {
        let mut left = idx;
        let mut right = idx + 99;

        while left <= right {
            let mid = (left + right) / 2;

            find(scan, min, max, mid, n);

            if max[mid] == n as i64 {
                *a2 = mid as i64;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    if *a1 != 0 && *a2 != 0 {
        true
    } else {
        false
    }
}

fn find_a1(
    scan: &mut UnsafeScanner<io::StdinLock<'_>>,
    min: &mut Vec<i64>,
    max: &mut Vec<i64>,
    a1: &mut i64,
    a2: &mut i64,
    k1: usize,
    k2: usize,
    n: usize,
) {
    let mut left = k1;
    let mut right = k2;

    while left <= right {
        let mid = (left + right) / 2;

        find(scan, min, max, mid, n);

        if check(scan, min, max, a1, a2, mid, n) {
            return;
        }

        find(scan, min, max, mid + 1, n);

        if check(scan, min, max, a1, a2, mid + 1, n) {
            return;
        }

        if max[mid] == n as i64 || max[mid + 1] == n as i64 {
            break;
        }

        if max[mid] < max[mid + 1] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
}

fn find_a2(
    scan: &mut UnsafeScanner<io::StdinLock<'_>>,
    min: &mut Vec<i64>,
    max: &mut Vec<i64>,
    a1: &mut i64,
    a2: &mut i64,
    k1: usize,
    k2: usize,
    n: usize,
) {
    let mut left = k1;
    let mut right = k2;

    while left <= right {
        let mid = (left + right) / 2;

        find(scan, min, max, mid, n);

        if check(scan, min, max, a1, a2, mid, n) {
            return;
        }

        find(scan, min, max, mid + 1, n);

        if check(scan, min, max, a1, a2, mid + 1, n) {
            return;
        }

        if max[mid] == 1 || max[mid + 1] == 1 {
            break;
        }

        if max[mid] > max[mid + 1] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
}

fn process_subtask4(
    scan: &mut UnsafeScanner<io::StdinLock<'_>>,
    a1: &mut i64,
    a2: &mut i64,
    n: usize,
) {
    let mut min = vec![0; 200_001];
    let mut max = vec![0; 200_001];

    find(scan, &mut min, &mut max, 1, n);

    if check(scan, &mut min, &mut max, a1, a2, 1, n) {
        return;
    }

    find(scan, &mut min, &mut max, 2, n);

    if check(scan, &mut min, &mut max, a1, a2, 2, n) {
        return;
    }

    if *a1 != 0 {
        find_a1(scan, &mut min, &mut max, a1, a2, *a1 as usize + 1, n, n);
        return;
    }

    if *a2 != 0 {
        find_a2(scan, &mut min, &mut max, a1, a2, *a2 as usize + 1, n, n);
        return;
    }

    if min[1] < min[2] {
        let mut left = 1;
        let mut right = n;
        let mut mid = 0;

        while left <= right {
            mid = (left + right) / 2;

            find(scan, &mut min, &mut max, mid, n);

            if check(scan, &mut min, &mut max, a1, a2, mid, n) {
                return;
            }

            find(scan, &mut min, &mut max, mid + 1, n);

            if check(scan, &mut min, &mut max, a1, a2, mid + 1, n) {
                return;
            }

            if min[mid] > min[mid + 1] {
                break;
            }

            if min[mid + 1] < min[1] {
                right = mid;
            } else if min[mid] > min[1] {
                left = mid + 1;
            }

            if *a1 != 0 {
                find_a1(scan, &mut min, &mut max, a1, a2, left, *a1 as usize - 1, n);
                return;
            }

            if *a2 != 0 {
                find_a2(scan, &mut min, &mut max, a1, a2, *a2 as usize + 1, right, n);
                return;
            }
        }

        let left2 = mid + 1;
        let right2 = right;

        right = mid;

        if *a2 == 0 {
            while left <= right {
                mid = (left + right) / 2;

                find(scan, &mut min, &mut max, mid, n);

                if check(scan, &mut min, &mut max, a1, a2, mid, n) {
                    return;
                }

                find(scan, &mut min, &mut max, mid + 1, n);

                if check(scan, &mut min, &mut max, a1, a2, mid + 1, n) {
                    return;
                }

                if *a2 != 0 {
                    break;
                }

                if max[mid] < max[mid + 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        left = left2;
        right = right2;

        if *a1 == 0 {
            while left <= right {
                mid = (left + right) / 2;

                find(scan, &mut min, &mut max, mid, n);

                if check(scan, &mut min, &mut max, a1, a2, mid, n) {
                    return;
                }

                find(scan, &mut min, &mut max, mid + 1, n);

                if check(scan, &mut min, &mut max, a1, a2, mid + 1, n) {
                    return;
                }

                if *a1 != 0 {
                    break;
                }

                if min[mid] > min[mid + 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
    } else {
        let mut left = 1;
        let mut right = 100_000;
        let mut mid = 0;

        while left <= right {
            mid = (left + right) / 2;

            find(scan, &mut min, &mut max, mid, n);

            if check(scan, &mut min, &mut max, a1, a2, mid, n) {
                return;
            }

            find(scan, &mut min, &mut max, mid + 1, n);

            if check(scan, &mut min, &mut max, a1, a2, mid + 1, n) {
                return;
            }

            if min[mid] < min[mid + 1] {
                break;
            }

            if min[mid + 1] > min[1] {
                right = mid;
            } else if min[mid] < min[1] {
                left = mid + 1;
            }

            if *a1 != 0 {
                find_a1(scan, &mut min, &mut max, a1, a2, *a1 as usize + 1, right, n);
                return;
            }

            if *a2 != 0 {
                find_a2(scan, &mut min, &mut max, a1, a2, left, *a2 as usize - 1, n);
                return;
            }
        }

        let left2 = mid + 1;
        let right2 = right;

        right = mid;

        if *a1 == 0 {
            while left <= right {
                mid = (left + right) / 2;

                find(scan, &mut min, &mut max, mid, n);

                if check(scan, &mut min, &mut max, a1, a2, mid, n) {
                    return;
                }

                find(scan, &mut min, &mut max, mid + 1, n);

                if check(scan, &mut min, &mut max, a1, a2, mid + 1, n) {
                    return;
                }

                if *a1 != 0 {
                    break;
                }

                if min[mid] > min[mid + 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        left = left2;
        right = right2;

        if *a2 == 0 {
            while left <= right {
                mid = (left + right) / 2;

                find(scan, &mut min, &mut max, mid, n);

                if check(scan, &mut min, &mut max, a1, a2, mid, n) {
                    return;
                }

                find(scan, &mut min, &mut max, mid + 1, n);

                if check(scan, &mut min, &mut max, a1, a2, mid + 1, n) {
                    return;
                }

                if *a2 != 0 {
                    break;
                }

                if max[mid] < max[mid + 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scan: UnsafeScanner<io::StdinLock<'_>> = UnsafeScanner::new(stdin.lock());

    let (n, m, _) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret1 = 100;
    let mut ret2 = 100;

    if n == 101 && m == 100 {
        let mut aa = 1;
        let mut bb = n;

        for i in 1..=100 {
            println!("? {i}");

            let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

            if aa == 1 && a != 1 {
                ret1 = i - 1;

                if ret1 == 0 {
                    ret1 = 101;
                }
            }

            if bb == n && b != n {
                ret2 = i - 1;

                if ret2 == 0 {
                    ret2 = 101;
                }
            }

            aa = a;
            bb = b;
        }

        println!("! {ret1} {ret2}");
    } else if n == 8000 && m == 100 {
        let mut aa = 0;
        let mut bb = 0;

        for i in 1..=80 {
            println!("? {}", (i - 1) * 100 + 1);

            let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

            if a == 1 {
                aa = (i - 1) * 100 + 1;
            }

            if b == n {
                bb = (i - 1) * 100 + 1;
            }
        }

        let mut left = aa;
        let mut right = aa + 99;
        let mut mid;

        while left <= right {
            mid = (left + right) / 2;

            println!("? {mid}");

            let (a, _) = (scan.token::<i64>(), scan.token::<i64>());

            if a != 1 {
                right = mid - 1;
            } else {
                left = mid + 1;
                ret1 = mid;
            }
        }

        left = bb;
        right = bb + 99;

        while left <= right {
            mid = (left + right) / 2;

            println!("? {mid}");

            let (_, b) = (scan.token::<i64>(), scan.token::<i64>());

            if b != n {
                right = mid - 1;
            } else {
                left = mid + 1;
                ret2 = mid;
            }
        }

        println!("! {ret1} {ret2}");
    } else if n == 2500 && m == 1 {
        let mut nums = vec![0; 2501];
        ret1 = 0;
        ret2 = 0;

        println!("? 1");
        (nums[1], _) = (scan.token::<i64>(), scan.token::<i64>());
        println!("? 2");
        (nums[2], _) = (scan.token::<i64>(), scan.token::<i64>());

        if nums[1] == n {
            ret2 = 1;
        }

        if nums[2] == n {
            ret2 = 2;
        }

        if nums[1] == 1 {
            ret1 = 1;
        }

        if nums[2] == 1 {
            ret1 = 2;
        }

        if ret1 != 0 && ret2 != 0 {
            // Do nothing
        } else if nums[1] < nums[2] {
            let mut tf = 0;
            let mut check1 = 2451;
            let mut check2 = 2451;

            for i in (51..=n).step_by(50) {
                println!("? {i}");
                (nums[i as usize], _) = (scan.token::<i64>(), scan.token::<i64>());

                if nums[i as usize] == n {
                    ret2 = i;
                }

                if nums[i as usize] == 1 {
                    ret1 = i;
                }

                if tf == 0 && nums[i as usize - 50] > nums[i as usize] {
                    tf = 1;
                    check1 = i - 50;
                } else if tf == 1 && nums[i as usize - 50] < nums[i as usize] {
                    tf = 2;
                    check2 = i - 50;
                }
            }

            if tf == 1 {
                if check1 == 1 {
                    let mut left = 1;
                    let mut right = 50;

                    while left + 3 <= right {
                        let llr = (2 * left + right) / 3;

                        if nums[llr] == 0 {
                            println!("? {llr}");
                            (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[llr] == n {
                            ret2 = llr as i64;
                            break;
                        }

                        let lrr = (left + 2 * right) / 3;

                        if nums[lrr] == 0 {
                            println!("? {lrr}");
                            (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[lrr] == n {
                            ret2 = lrr as i64;
                            break;
                        }

                        if nums[llr] > nums[lrr] {
                            right = lrr;
                        } else {
                            left = llr;
                        }
                    }

                    if ret2 == 0 {
                        for i in left..=right {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == n {
                                ret2 = i as i64;
                            }
                        }
                    }

                    if nums[check2 as usize + 1] == 0 {
                        println!("? {}", check2 + 1);
                        (nums[check2 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize + 1] == 1 {
                        ret1 = check2 + 1;
                    }

                    if ret1 != 0 {
                        // Do nothing
                    } else if nums[check2 as usize] > nums[check2 as usize + 1] {
                        let mut left = check2 as usize;
                        let mut right = check2 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check2 as usize - 50;
                        let mut right = check2 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    }
                } else {
                    if nums[check1 as usize + 1] == 0 {
                        println!("? {}", check1 + 1);
                        (nums[check1 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check1 as usize + 1] == n {
                        ret2 = check1 + 1;
                    }

                    if ret2 != 0 {
                        // Do nothing
                    } else if nums[check1 as usize] < nums[check1 as usize + 1] {
                        let mut left = check1 as usize;
                        let mut right = check1 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check1 as usize - 50;
                        let mut right = check1 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    }

                    if nums[check2 as usize + 1] == 0 {
                        println!("? {}", check2 + 1);
                        (nums[check2 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize + 1] == 1 {
                        ret1 = check2 + 1;
                    }

                    if ret1 != 0 {
                        // Do nothing
                    } else if nums[check2 as usize] > nums[check2 as usize + 1] {
                        let mut left = check2 as usize;
                        let mut right = check2 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check2 as usize - 50;
                        let mut right = check2 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    }
                }
            } else if tf == 2 {
                if check1 + 50 == check2 {
                    if nums[check1 as usize + 1] == 0 {
                        println!("? {}", check1 + 1);
                        (nums[check1 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check1 as usize + 1] == n {
                        ret2 = check1 + 1;
                    }

                    if nums[check2 as usize + 1] == 0 {
                        println!("? {}", check2 - 1);
                        (nums[check2 as usize - 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize - 1] == 1 {
                        ret1 = check2 - 1;
                    }

                    if nums[check1 as usize] < nums[check1 as usize + 1]
                        && nums[check2 as usize - 1] < nums[check2 as usize]
                    {
                        if ret1 == 0 {
                            ret1 = check2 - 2;
                        }

                        if ret2 == 0 {
                            ret2 = check1 - 2;
                        }

                        for i in (check1 as usize + 2)..(check2 as usize - 2) {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == 1 {
                                ret1 = i as i64;
                            }

                            if nums[i] == n {
                                ret2 = i as i64;
                            }
                        }
                    } else {
                        if ret2 != 0 {
                            // Do nothing
                        } else if nums[check1 as usize] < nums[check1 as usize + 1] {
                            let mut left = check1 as usize;
                            let mut right = check1 as usize + 49;

                            while left + 3 <= right {
                                let llr = (2 * left + right) / 3;

                                if nums[llr] == 0 {
                                    println!("? {llr}");
                                    (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[llr] == n {
                                    ret2 = llr as i64;
                                    break;
                                }

                                let lrr = (left + 2 * right) / 3;

                                if nums[lrr] == 0 {
                                    println!("? {lrr}");
                                    (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[lrr] == n {
                                    ret2 = lrr as i64;
                                    break;
                                }

                                if nums[llr] > nums[lrr] {
                                    right = lrr;
                                } else {
                                    left = llr;
                                }
                            }

                            if ret2 == 0 {
                                for i in left..=right {
                                    if nums[i] == 0 {
                                        println!("? {i}");
                                        (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                    }

                                    if nums[i] == n {
                                        ret2 = i as i64;
                                    }
                                }
                            }
                        } else {
                            let mut left = check1 as usize - 50;
                            let mut right = check1 as usize - 1;

                            while left + 3 <= right {
                                let llr = (2 * left + right) / 3;

                                if nums[llr] == 0 {
                                    println!("? {llr}");
                                    (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[llr] == n {
                                    ret2 = llr as i64;
                                    break;
                                }

                                let lrr = (left + 2 * right) / 3;

                                if nums[lrr] == 0 {
                                    println!("? {lrr}");
                                    (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[lrr] == n {
                                    ret2 = lrr as i64;
                                    break;
                                }

                                if nums[llr] > nums[lrr] {
                                    right = lrr;
                                } else {
                                    left = llr;
                                }
                            }

                            if ret2 == 0 {
                                for i in left..=right {
                                    if nums[i] == 0 {
                                        println!("? {i}");
                                        (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                    }

                                    if nums[i] == n {
                                        ret2 = i as i64;
                                    }
                                }
                            }
                        }

                        if ret1 != 0 {
                            // Do nothing
                        } else if nums[check2 as usize - 1] < nums[check2 as usize] {
                            let mut left = check2 as usize - 50;
                            let mut right = check2 as usize - 1;

                            while left + 3 <= right {
                                let llr = (2 * left + right) / 3;

                                if nums[llr] == 0 {
                                    println!("? {llr}");
                                    (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[llr] == 1 {
                                    ret1 = llr as i64;
                                    break;
                                }

                                let lrr = (left + 2 * right) / 3;

                                if nums[lrr] == 0 {
                                    println!("? {lrr}");
                                    (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[lrr] == 1 {
                                    ret1 = lrr as i64;
                                    break;
                                }

                                if nums[llr] < nums[lrr] {
                                    right = lrr;
                                } else {
                                    left = llr;
                                }
                            }

                            if ret1 == 0 {
                                for i in left..=right {
                                    if nums[i] == 0 {
                                        println!("? {i}");
                                        (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                    }

                                    if nums[i] == 1 {
                                        ret1 = i as i64;
                                    }
                                }
                            }
                        } else {
                            let mut left = check2 as usize;
                            let mut right = check2 as usize + 49;

                            while left + 3 <= right {
                                let llr = (2 * left + right) / 3;

                                if nums[llr] == 0 {
                                    println!("? {llr}");
                                    (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[llr] == 1 {
                                    ret1 = llr as i64;
                                    break;
                                }

                                let lrr = (left + 2 * right) / 3;

                                if nums[lrr] == 0 {
                                    println!("? {lrr}");
                                    (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[lrr] == 1 {
                                    ret1 = lrr as i64;
                                    break;
                                }

                                if nums[llr] < nums[lrr] {
                                    right = lrr;
                                } else {
                                    left = llr;
                                }
                            }

                            if ret1 == 0 {
                                for i in left..=right {
                                    if nums[i] == 0 {
                                        println!("? {i}");
                                        (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                    }

                                    if nums[i] == 1 {
                                        ret1 = i as i64;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if nums[check1 as usize + 1] == 0 {
                        println!("? {}", check1 + 1);
                        (nums[check1 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check1 as usize + 1] == n {
                        ret2 = check1 + 1;
                    }

                    if nums[check2 as usize - 1] == 0 {
                        println!("? {}", check2 - 1);
                        (nums[check2 as usize - 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize - 1] == 1 {
                        ret1 = check2 + 1;
                    }

                    if ret2 != 0 {
                        // Do nothing
                    } else if nums[check1 as usize] < nums[check1 as usize + 1] {
                        let mut left = check1 as usize;
                        let mut right = check1 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check1 as usize - 50;
                        let mut right = check1 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    }

                    if ret1 != 0 {
                        // Do nothing
                    } else if nums[check2 as usize - 1] < nums[check2 as usize] {
                        let mut left = check2 as usize - 50;
                        let mut right = check2 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check2 as usize;
                        let mut right = check2 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    }
                }
            } else {
                if nums[2452] == 0 {
                    println!("? 2452");
                    (nums[2452], _) = (scan.token::<i64>(), scan.token::<i64>());
                }

                if nums[2451] < nums[2452] {
                    if ret1 == 0 {
                        ret1 = 2500;
                    }

                    if ret2 == 0 {
                        ret2 = 2500;
                    }

                    for i in 2453..2500 {
                        if nums[i] == 0 {
                            println!("? {i}");
                            (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[i] == n {
                            ret2 = i as i64;
                        } else if nums[i] == 1 {
                            ret1 = i as i64;
                        }
                    }
                } else {
                    let mut left = check1 as usize - 50;
                    let mut right = check1 as usize - 1;

                    while left + 3 <= right {
                        let llr = (2 * left + right) / 3;

                        if nums[llr] == 0 {
                            println!("? {llr}");
                            (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[llr] == n {
                            ret2 = llr as i64;
                            break;
                        }

                        let lrr = (left + 2 * right) / 3;

                        if nums[lrr] == 0 {
                            println!("? {lrr}");
                            (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[lrr] == n {
                            ret2 = lrr as i64;
                            break;
                        }

                        if nums[llr] > nums[lrr] {
                            right = lrr;
                        } else {
                            left = llr;
                        }
                    }

                    if ret2 == 0 {
                        for i in left..=right {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == n {
                                ret2 = i as i64;
                            }
                        }
                    }

                    let mut left = check2 as usize;
                    let mut right = check2 as usize + 49;

                    while left + 3 <= right {
                        let llr = (2 * left + right) / 3;

                        if nums[llr] == 0 {
                            println!("? {llr}");
                            (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[llr] == 1 {
                            ret1 = llr as i64;
                            break;
                        }

                        let lrr = (left + 2 * right) / 3;

                        if nums[lrr] == 0 {
                            println!("? {lrr}");
                            (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[lrr] == 1 {
                            ret1 = lrr as i64;
                            break;
                        }

                        if nums[llr] < nums[lrr] {
                            right = lrr;
                        } else {
                            left = llr;
                        }
                    }

                    if ret1 == 0 {
                        for i in left..=right {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == 1 {
                                ret1 = i as i64;
                            }
                        }
                    }
                }
            }
        } else {
            let mut tf = 0;
            let mut check1 = 2451;
            let mut check2 = 2451;

            for i in (51..=n).step_by(50) {
                println!("? {i}");
                (nums[i as usize], _) = (scan.token::<i64>(), scan.token::<i64>());

                if nums[i as usize] == n {
                    ret2 = i;
                }

                if nums[i as usize] == 1 {
                    ret1 = i;
                }

                if tf == 0 && nums[i as usize - 50] < nums[i as usize] {
                    tf = 1;
                    check1 = i - 50;
                } else if tf == 1 && nums[i as usize - 50] > nums[i as usize] {
                    tf = 2;
                    check2 = i - 50;
                }
            }

            if tf == 1 {
                if check1 == 1 {
                    let mut left = 1;
                    let mut right = 50;

                    while left + 3 <= right {
                        let llr = (2 * left + right) / 3;

                        if nums[llr] == 0 {
                            println!("? {llr}");
                            (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[llr] == 1 {
                            ret1 = llr as i64;
                            break;
                        }

                        let lrr = (left + 2 * right) / 3;

                        if nums[lrr] == 0 {
                            println!("? {lrr}");
                            (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[lrr] == 1 {
                            ret1 = lrr as i64;
                            break;
                        }

                        if nums[llr] < nums[lrr] {
                            right = lrr;
                        } else {
                            left = llr;
                        }
                    }

                    if ret1 == 0 {
                        for i in left..=right {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == 1 {
                                ret1 = i as i64;
                            }
                        }
                    }

                    if nums[2452] == 0 {
                        println!("? 2452");
                        (nums[2452], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[2452] == n {
                        ret2 = 2452;
                    }

                    if ret2 != 0 {
                        // Do nothing
                    } else if nums[2451] < nums[2452] {
                        let mut left = 2451;
                        let mut right = 2500;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = 2401;
                        let mut right = 2450;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    }
                } else {
                    if nums[check1 as usize + 1] == 0 {
                        println!("? {}", check1 + 1);
                        (nums[check1 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check1 as usize + 1] == n {
                        ret1 = check1 + 1;
                    }

                    if ret1 != 0 {
                        // Do nothing
                    } else if nums[check1 as usize] > nums[check1 as usize + 1] {
                        let mut left = check1 as usize;
                        let mut right = check1 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check1 as usize - 50;
                        let mut right = check1 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    }

                    if nums[check2 as usize + 1] == 0 {
                        println!("? {}", check2 + 1);
                        (nums[check2 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize + 1] == n {
                        ret2 = check2 + 1;
                    }

                    if ret2 != 0 {
                        // Do nothing
                    } else if nums[check2 as usize] < nums[check2 as usize + 1] {
                        let mut left = check2 as usize;
                        let mut right = check2 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check2 as usize - 50;
                        let mut right = check2 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    }
                }
            } else if tf == 2 {
                if check1 + 50 == check2 {
                    if nums[check1 as usize + 1] == 0 {
                        println!("? {}", check1 + 1);
                        (nums[check1 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check1 as usize + 1] == 1 {
                        ret1 = check1 + 1;
                    }

                    if nums[check2 as usize - 1] == 0 {
                        println!("? {}", check2 - 1);
                        (nums[check2 as usize - 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize - 1] == n {
                        ret2 = check2 - 1;
                    }

                    if nums[check1 as usize] > nums[check1 as usize + 1]
                        && nums[check2 as usize - 1] > nums[check2 as usize]
                    {
                        if ret1 == 0 {
                            ret1 = check2 - 2;
                        }

                        if ret2 == 0 {
                            ret2 = check2 - 2;
                        }

                        for i in (check1 as usize + 2)..(check2 as usize - 2) {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == 1 {
                                ret1 = i as i64;
                            }

                            if nums[i] == n {
                                ret2 = i as i64;
                            }
                        }
                    } else {
                        if ret1 != 0 {
                            // Do nothing
                        } else if nums[check1 as usize] > nums[check1 as usize + 1] {
                            let mut left = check1 as usize;
                            let mut right = check1 as usize + 49;

                            while left + 3 <= right {
                                let llr = (2 * left + right) / 3;

                                if nums[llr] == 0 {
                                    println!("? {llr}");
                                    (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[llr] == 1 {
                                    ret1 = llr as i64;
                                    break;
                                }

                                let lrr = (left + 2 * right) / 3;

                                if nums[lrr] == 0 {
                                    println!("? {lrr}");
                                    (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[lrr] == 1 {
                                    ret1 = lrr as i64;
                                    break;
                                }

                                if nums[llr] < nums[lrr] {
                                    right = lrr;
                                } else {
                                    left = llr;
                                }
                            }

                            if ret1 == 0 {
                                for i in left..=right {
                                    if nums[i] == 0 {
                                        println!("? {i}");
                                        (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                    }

                                    if nums[i] == 1 {
                                        ret1 = i as i64;
                                    }
                                }
                            }
                        } else {
                            let mut left = check1 as usize - 50;
                            let mut right = check1 as usize - 1;

                            while left + 3 <= right {
                                let llr = (2 * left + right) / 3;

                                if nums[llr] == 0 {
                                    println!("? {llr}");
                                    (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[llr] == 1 {
                                    ret1 = llr as i64;
                                    break;
                                }

                                let lrr = (left + 2 * right) / 3;

                                if nums[lrr] == 0 {
                                    println!("? {lrr}");
                                    (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[lrr] == 1 {
                                    ret1 = lrr as i64;
                                    break;
                                }

                                if nums[llr] < nums[lrr] {
                                    right = lrr;
                                } else {
                                    left = llr;
                                }
                            }

                            if ret1 == 0 {
                                for i in left..=right {
                                    if nums[i] == 0 {
                                        println!("? {i}");
                                        (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                    }

                                    if nums[i] == 1 {
                                        ret1 = i as i64;
                                    }
                                }
                            }
                        }
                    }

                    if ret2 != 0 {
                        // Do nothing
                    } else if nums[check2 as usize - 1] > nums[check2 as usize] {
                        let mut left = check2 as usize - 50;
                        let mut right = check2 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check2 as usize;
                        let mut right = check2 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    }
                } else {
                    if nums[check1 as usize + 1] == 0 {
                        println!("? {}", check1 + 1);
                        (nums[check1 as usize + 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check1 as usize + 1] == 1 {
                        ret1 = check1 + 1;
                    }

                    if nums[check2 as usize - 1] == 0 {
                        println!("? {}", check2 - 1);
                        (nums[check2 as usize - 1], _) = (scan.token::<i64>(), scan.token::<i64>());
                    }

                    if nums[check2 as usize - 1] == n {
                        ret2 = check2 - 1;
                    }

                    if ret1 != 0 {
                        // Do nothing
                    } else if nums[check1 as usize] > nums[check1 as usize + 1] {
                        let mut left = check1 as usize;
                        let mut right = check1 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check1 as usize - 50;
                        let mut right = check1 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == 1 {
                                ret1 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == 1 {
                                ret1 = lrr as i64;
                                break;
                            }

                            if nums[llr] < nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret1 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == 1 {
                                    ret1 = i as i64;
                                }
                            }
                        }
                    }

                    if ret2 != 0 {
                        // Do nothing
                    } else if nums[check2 as usize - 1] > nums[check2 as usize] {
                        let mut left = check2 as usize - 50;
                        let mut right = check2 as usize - 1;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    } else {
                        let mut left = check2 as usize;
                        let mut right = check2 as usize + 49;

                        while left + 3 <= right {
                            let llr = (2 * left + right) / 3;

                            if nums[llr] == 0 {
                                println!("? {llr}");
                                (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[llr] == n {
                                ret2 = llr as i64;
                                break;
                            }

                            let lrr = (left + 2 * right) / 3;

                            if nums[lrr] == 0 {
                                println!("? {lrr}");
                                (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[lrr] == n {
                                ret2 = lrr as i64;
                                break;
                            }

                            if nums[llr] > nums[lrr] {
                                right = lrr;
                            } else {
                                left = llr;
                            }
                        }

                        if ret2 == 0 {
                            for i in left..=right {
                                if nums[i] == 0 {
                                    println!("? {i}");
                                    (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                                }

                                if nums[i] == n {
                                    ret2 = i as i64;
                                }
                            }
                        }
                    }
                }
            } else {
                if nums[2452] == 0 {
                    println!("? 2452");
                    (nums[2452], _) = (scan.token::<i64>(), scan.token::<i64>());
                }

                if nums[2451] > nums[2452] {
                    if ret1 == 0 {
                        ret1 = 2500;
                    }

                    if ret2 == 0 {
                        ret2 = 2500;
                    }

                    for i in 2453..2500 {
                        if nums[i] == 0 {
                            println!("? {i}");
                            (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[i] == n {
                            ret2 = i as i64;
                        } else if nums[i] == 1 {
                            ret1 = i as i64;
                        }
                    }
                } else {
                    let mut left = 2401;
                    let mut right = 2450;

                    while left + 3 <= right {
                        let llr = (2 * left + right) / 3;

                        if nums[llr] == 0 {
                            println!("? {llr}");
                            (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[llr] == 1 {
                            ret1 = llr as i64;
                            break;
                        }

                        let lrr = (left + 2 * right) / 3;

                        if nums[lrr] == 0 {
                            println!("? {lrr}");
                            (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[lrr] == 1 {
                            ret1 = lrr as i64;
                            break;
                        }

                        if nums[llr] < nums[lrr] {
                            right = lrr;
                        } else {
                            left = llr;
                        }
                    }

                    if ret1 == 0 {
                        for i in left..=right {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == 1 {
                                ret1 = i as i64;
                            }
                        }
                    }

                    let mut left = 2451;
                    let mut right = 2500;

                    while left + 3 <= right {
                        let llr = (2 * left + right) / 3;

                        if nums[llr] == 0 {
                            println!("? {llr}");
                            (nums[llr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[llr] == n {
                            ret2 = llr as i64;
                            break;
                        }

                        let lrr = (left + 2 * right) / 3;

                        if nums[lrr] == 0 {
                            println!("? {lrr}");
                            (nums[lrr], _) = (scan.token::<i64>(), scan.token::<i64>());
                        }

                        if nums[lrr] == n {
                            ret2 = lrr as i64;
                            break;
                        }

                        if nums[llr] > nums[lrr] {
                            right = lrr;
                        } else {
                            left = llr;
                        }
                    }

                    if ret2 == 0 {
                        for i in left..=right {
                            if nums[i] == 0 {
                                println!("? {i}");
                                (nums[i], _) = (scan.token::<i64>(), scan.token::<i64>());
                            }

                            if nums[i] == n {
                                ret2 = i as i64;
                            }
                        }
                    }
                }
            }
        }

        println!("! {ret1} {ret2}");
    } else {
        let mut a1 = 0;
        let mut a2 = 0;

        process_subtask4(&mut scan, &mut a1, &mut a2, n as usize);

        println!(
            "! {} {}",
            if a1 > 100_000 { a1 - 100_000 } else { a1 },
            if a2 > 100_000 { a2 - 100_000 } else { a2 }
        );
    }
}
