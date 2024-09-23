impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        let mut lo = 1;
        let mut hi = *position.iter().max().unwrap();

        position.sort_unstable();

        while lo < hi {
            let force = (lo + hi + 1) / 2;
            let mut last = -force;
            let mut count = 0;

            for i in 0..position.len() {
                if position[i] - last >= force {
                    last = position[i];
                    count += 1;
                }
            }

            if count >= m {
                lo = force;
            } else {
                hi = force - 1;
            }
        }

        hi
    }
}
