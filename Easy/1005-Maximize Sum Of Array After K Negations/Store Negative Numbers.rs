impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut cnt_neg = [0; 101];
        let mut sum = 0;
        let mut min_abs = 100;

        for n in a {
            if n >= 0 {
                sum += n;
            } else {
                cnt_neg[-n as usize] += 1;
            }
            min_abs = min_abs.min(n.abs());
        }

        for i in (1..101).rev() {
            if k > 0 {
                if cnt_neg[i as usize] <= k {
                    sum += i * cnt_neg[i as usize];
                    k -= cnt_neg[i as usize];
                } else {
                    sum += i * (2 * k - cnt_neg[i as usize]);
                    k = 0;
                }
            } else {
                sum += -i * cnt_neg[i as usize];
            }
        }

        if k % 2 == 1 {
            sum -= 2 * min_abs;
        }

        sum
    }
}
