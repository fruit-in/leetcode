impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let stations = stations.into_iter().map(|p| p as i64).collect::<Vec<_>>();
        let r = r as usize;
        let k = k as i64;
        let n = stations.len();
        let mut lo = i64::MAX;
        let mut hi = i64::MIN;

        for i in 0..n {
            lo = lo.min(stations[i]);
            hi = hi.max(stations[i] * (r as i64 * 2 + 1) + k);
        }

        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            let mut new_stations = stations.clone();
            let mut power = (0..r).map(|i| stations[i]).sum::<i64>();
            let mut build = 0;

            for i in 0..stations.len() {
                if i > r {
                    power -= new_stations[i - r - 1];
                }
                if i + r < n {
                    power += new_stations[i + r];
                }
                if power < mid {
                    new_stations[(i + r).min(n - 1)] += mid - power;
                    build += mid - power;
                    power = mid;
                }

                if build > k {
                    break;
                }
            }

            if build <= k {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }
}
