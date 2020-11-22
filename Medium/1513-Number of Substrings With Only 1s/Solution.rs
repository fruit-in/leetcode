impl Solution {
    pub fn num_sub(s: String) -> i32 {
        (s.split('0')
            .map(|ones| ones.len())
            .fold(0, |acc, x| acc + (x + 1) * x / 2)
            % 1000000007) as i32
    }
}
