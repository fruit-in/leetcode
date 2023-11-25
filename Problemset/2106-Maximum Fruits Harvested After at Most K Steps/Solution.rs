impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut prefix_sum = vec![(-1, 0)];
        let mut ret = 0;

        for fruit in &fruits {
            if (fruit[0] - start_pos).abs() <= k {
                let amount = prefix_sum.last().unwrap().1;
                prefix_sum.push((fruit[0], fruit[1] + amount));
            }
        }

        for i in 1..prefix_sum.len() {
            if prefix_sum[i].0 < start_pos {
                let start = prefix_sum[i].0;
                let end = start_pos.max(k - start_pos + 2 * start);
                let j = prefix_sum.binary_search(&(end, i32::MAX)).unwrap_err() - 1;

                ret = ret.max(prefix_sum[j].1 - prefix_sum[i - 1].1);
            } else {
                let end = prefix_sum[i].0;
                let start = start_pos.min(2 * end - k - start_pos).max(0);
                let j = prefix_sum.binary_search(&(start, 0)).unwrap_err();

                ret = ret.max(prefix_sum[i].1 - prefix_sum[j - 1].1);
            }
        }

        ret
    }
}
