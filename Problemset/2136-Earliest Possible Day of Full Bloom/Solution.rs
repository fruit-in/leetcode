impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut seeds = plant_time.iter().zip(grow_time.iter()).collect::<Vec<_>>();
        let mut day = 0;
        let mut ret = 0;

        seeds.sort_unstable_by_key(|&(_, g)| -g);

        for (p, g) in seeds {
            day += p;
            ret = ret.max(day + g);
        }

        ret
    }
}
