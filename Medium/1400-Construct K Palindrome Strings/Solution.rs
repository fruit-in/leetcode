impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let count = s.bytes().fold(0i32, |acc, x| acc ^ (1 << (x - b'a')));

        count.count_ones() as i32 <= k && s.len() as i32 >= k
    }
}
