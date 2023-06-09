impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut char_vals = (1..=26).collect::<Vec<_>>();
        let mut prefix_sum = 0;
        let mut min_sum = 0;
        let mut ret = 0;

        for (i, c) in chars.bytes().enumerate() {
            char_vals[(c - b'a') as usize] = vals[i];
        }

        for c in s.bytes() {
            prefix_sum += char_vals[(c - b'a') as usize];
            min_sum = min_sum.min(prefix_sum);
            ret = ret.max(prefix_sum - min_sum);
        }

        ret
    }
}
