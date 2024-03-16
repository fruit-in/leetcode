impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| (p[1], -p[0]));
        let mut right = pairs[0][1];
        let mut ret = 1;

        for i in 1..pairs.len() {
            if pairs[i][0] > right {
                right = pairs[i][1];
                ret += 1;
            }
        }

        ret
    }
}
