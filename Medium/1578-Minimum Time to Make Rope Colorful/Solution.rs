impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut ret = 0;

        for i in 1..s.len() {
            if s[i] == s[prev] {
                if cost[i] < cost[prev] {
                    ret += cost[i];
                    continue;
                }
                ret += cost[prev];
            }
            prev = i;
        }

        ret
    }
}
