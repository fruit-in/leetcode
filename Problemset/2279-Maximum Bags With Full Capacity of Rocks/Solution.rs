impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remains = (0..rocks.len())
            .map(|i| capacity[i] - rocks[i])
            .collect::<Vec<_>>();
        let mut additional_rocks = additional_rocks;
        let mut ret = 0;
        remains.sort_unstable();

        for remain in remains {
            if remain > additional_rocks {
                break;
            }

            additional_rocks -= remain;
            ret += 1;
        }

        ret
    }
}
