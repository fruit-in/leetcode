use std::collections::HashSet;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut roads_set = HashSet::new();
        let mut count = vec![0; n];
        let mut ret = 0;

        for road in &roads {
            let a = (road[0]).min(road[1]) as usize;
            let b = (road[0]).max(road[1]) as usize;

            roads_set.insert((a, b));
            count[a] += 1;
            count[b] += 1;
        }

        for a in 0..n {
            for b in a + 1..n {
                ret = ret.max(count[a] + count[b] - roads_set.contains(&(a, b)) as i32);
            }
        }

        ret
    }
}
