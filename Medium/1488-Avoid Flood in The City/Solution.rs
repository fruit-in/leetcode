use std::collections::HashMap;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut dry_days = vec![];
        let mut rain_days = HashMap::new();
        let mut ans = vec![-1; rains.len()];

        for i in 0..rains.len() {
            if rains[i] == 0 {
                dry_days.push(i);
            } else if let Some(j) = rain_days.insert(rains[i], i) {
                match dry_days.binary_search(&j) {
                    Err(k) if k == dry_days.len() => return vec![],
                    Err(k) => {
                        ans[dry_days[k]] = rains[i];
                        dry_days.remove(k);
                    }
                    _ => (),
                }
            }
        }

        for i in dry_days {
            ans[i] = 1;
        }

        ans
    }
}
