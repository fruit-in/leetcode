impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut l_sum = 0i64;
        let mut r_sum = 0i64;
        let mut l_max_sum = 0i64;
        let mut r_max_sum = 0i64;
        let mut l_min_sum = 0i64;
        let mut ret = 0i64;

        for i in 0..arr.len() {
            l_sum += arr[i] as i64;
            r_sum += arr[arr.len() - 1 - i] as i64;
            l_max_sum = l_max_sum.max(l_sum);
            r_max_sum = r_max_sum.max(r_sum);
            l_min_sum = l_min_sum.min(l_sum);
            ret = ret.max(l_sum - l_min_sum);
        }

        match k {
            1 => ret as i32,
            _ => {
                (ret.max(l_sum.max(0) * (k as i64 - 2) + l_max_sum + r_max_sum) % 1_000_000_007)
                    as i32
            }
        }
    }
}
