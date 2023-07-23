impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let max_weight = *weights.iter().max().unwrap();
        let sum_weight = weights.iter().sum::<i32>();
        let capacities = (max_weight..=sum_weight).collect::<Vec<_>>();

        match capacities.binary_search_by_key(&false, |&c| Self::shipped_in_time(&weights, c, days))
        {
            Ok(c) => c as i32 + 1 + max_weight,
            Err(c) => c as i32 + max_weight,
        }
    }

    fn shipped_in_time(weights: &[i32], capacity: i32, days: i32) -> bool {
        let mut remain = 0;
        let mut spend = 0;

        for &w in weights {
            if remain < w {
                remain = capacity;
                spend += 1;
            }
            remain -= w;
        }

        spend <= days
    }
}
