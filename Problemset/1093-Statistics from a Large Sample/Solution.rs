impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        vec![
            Self::minimum(&count),
            Self::maximum(&count),
            Self::mean(&count),
            Self::median(&count),
            Self::mode(&count),
        ]
    }

    pub fn minimum(count: &[i32]) -> f64 {
        count
            .iter()
            .enumerate()
            .filter(|(_, c)| **c > 0)
            .next()
            .unwrap()
            .0 as f64
    }

    pub fn maximum(count: &[i32]) -> f64 {
        count
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, c)| **c > 0)
            .next()
            .unwrap()
            .0 as f64
    }

    pub fn mean(count: &[i32]) -> f64 {
        let sum = count
            .iter()
            .enumerate()
            .map(|(k, c)| k as u64 * *c as u64)
            .sum::<u64>();
        let count_sum = count.iter().sum::<i32>();

        sum as f64 / count_sum as f64
    }

    pub fn median(count: &[i32]) -> f64 {
        let mut l = 0;
        let mut r = 255;
        let mut l_count = count[l];
        let mut r_count = count[r];

        while l < r {
            if l_count > r_count {
                l_count -= r_count;
                r -= 1;
                r_count = count[r];
            } else if l_count < r_count {
                r_count -= l_count;
                l += 1;
                l_count = count[l];
            } else {
                l += 1;
                l_count = count[l];
                r -= 1;
                r_count = count[r];
            }
        }

        (l + r) as f64 / 2.
    }

    pub fn mode(count: &[i32]) -> f64 {
        count.iter().enumerate().max_by_key(|(_, c)| *c).unwrap().0 as f64
    }
}
