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

#[derive(Debug, Default, Clone, Copy)]
struct Sphere {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

const BUCKET_SIZE: usize = 1_573_939;

#[inline]
fn hash(x: i64, y: i64, z: i64) -> usize {
    let mut h = x;

    h = h.wrapping_mul(31).wrapping_add(y);
    h = h.wrapping_mul(31).wrapping_add(z);
    h = h.wrapping_rem(BUCKET_SIZE as i64);

    h as usize
}

/// Count pairs of spheres that "touch" when their radii are increased by query/2,
/// and collect the corresponding distance (after ceiling) into `ret`.
///
/// The function returns the total number of pairs found (up to k pairs).
///
/// # Parameters
/// - `spheres`: Slice of spheres (assumed to be sorted in descending order by radius).
/// - `ret`: Vector to collect the computed distances (ceiled).
/// - `query`: The extra distance parameter (the binary search variable).
/// - `k`: The target number of pairs we are interested in.
///
/// # Explanation
/// We use a spatial grid (with bucket hashing) to only check nearby spheres.
/// For each sphere, we update the grid if a smaller cell width is possible based on the current sphere's effective diameter.
/// Then, we check the current sphere against spheres in adjacent grid cells.
/// If the Euclidean distance between sphere centers is within the effective sum of radii (each increased by query/2),
/// we compute the extra gap (distance minus the sum of original radii), take its ceiling, and record it.
fn count_pairs(spheres: &[Sphere], ret: &mut Vec<i64>, query: i64, k: usize) -> usize {
    let mut buckets = vec![Vec::new(); BUCKET_SIZE];
    let mut width = 1_000_000_000;
    let mut cnt = 0;

    for i in 0..spheres.len() {
        // Compute the effective diameter for the current sphere.
        // In original units, effective diameter = 2*r + query
        let val = spheres[i].r * 2 + query;

        // If the new grid cell size (val * 2) is smaller than the current width,
        // rebuild the grid with the smaller cell size.
        if val * 2 < width {
            for bucket in buckets.iter_mut() {
                bucket.clear();
            }

            width = val;

            // Insert remaining spheres (with index > i) into the grid.
            for j in (i + 1)..spheres.len() {
                let x = spheres[j].x / width;
                let y = spheres[j].y / width;
                let z = spheres[j].z / width;

                if x < 0 || y < 0 || z < 0 {
                    continue;
                }

                let h = hash(x, y, z);
                buckets[h].push(j);
            }
        }

        // Compute grid indices for the current sphere.
        let idx_x = spheres[i].x / width;
        let idx_y = spheres[i].y / width;
        let idx_z = spheres[i].z / width;

        // Check all adjacent cells (including the current one) in the grid.
        if idx_x < 0 || idx_y < 0 || idx_z < 0 {
            continue;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let x_next = idx_x + dx;
                    let y_next = idx_y + dy;
                    let z_next = idx_z + dz;

                    if x_next < 0 || y_next < 0 || z_next < 0 {
                        continue;
                    }

                    let h = hash(x_next, y_next, z_next);

                    // Process each candidate sphere in the bucket.
                    for &j in buckets[h].iter() {
                        if j <= i {
                            continue;
                        }

                        let dx = spheres[i].x - spheres[j].x;
                        let dy = spheres[i].y - spheres[j].y;
                        let dz = spheres[i].z - spheres[j].z;
                        let dist = dx * dx + dy * dy + dz * dz;

                        // Compute effective sum of radii when each sphere's radius is increased by query/2.
                        // In original units, effective radius becomes r + query/2,
                        // so the sum is: r_i + r_j + query.
                        let sum_r = spheres[i].r + spheres[j].r + query;
                        let bound = sum_r * sum_r;

                        // If the squared distance exceeds the squared effective sum, skip.
                        if dist > bound {
                            continue;
                        }

                        // Compute the gap between the actual distance and the sum of original radii.
                        let gap =
                            ((dist as f64).sqrt() - (spheres[i].r + spheres[j].r) as f64).max(0.0);
                        // Ceiling the gap gives us the final answer for this pair.
                        let val = gap.ceil() as i64;

                        if ret.len() < k {
                            ret.push(val);
                        }

                        cnt += 1;

                        if cnt >= k && ret.len() >= k {
                            return cnt;
                        }
                    }
                }
            }
        }
    }

    cnt
}

// Reference: 37th Petrozavodsk Programming Camp, Summer 2019, Day 1: Songyang Chen Contest 2 Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
        let mut spheres = vec![Sphere::default(); n];

        for i in 0..n {
            let (x, y, z, r) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
            spheres[i] = Sphere { x, y, z, r };
        }

        // Sort spheres in descending order by radius.
        // This ordering helps the counting process by ensuring that larger spheres are processed first.
        spheres.sort_by(|a, b| b.r.cmp(&a.r));

        let mut left = 0;
        let mut right = 2_000_000;

        while left < right {
            let mid = (left + right) / 2;
            let mut ret = Vec::new();
            let center = count_pairs(&spheres, &mut ret, mid, k);

            // If there are at least k touching pairs, try a smaller query value.
            if center >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        let query = if left > 0 { left - 1 } else { 0 };
        let mut ret = Vec::new();

        count_pairs(&spheres, &mut ret, query, k);

        ret.sort_unstable();

        let mut idx = 0;

        // Output the smallest k distances.
        // If we have fewer than k computed distances, output the binary search result 'left' for the remaining ones.
        while idx < k {
            writeln!(out, "{}", if idx < ret.len() { ret[idx] } else { left }).unwrap();
            idx += 1;
        }
    }
}
