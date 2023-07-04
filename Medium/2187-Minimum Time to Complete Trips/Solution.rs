impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let mut l = 1_i64;
        let mut r = total_trips * *time.iter().min().unwrap() as i64;

        while l < r {
            let m = (l + r) / 2;
            let trips = time.iter().map(|&t| m / t as i64).sum::<i64>();

            if trips < total_trips {
                l = m + 1;
            } else {
                r = m;
            }
        }

        r
    }
}
