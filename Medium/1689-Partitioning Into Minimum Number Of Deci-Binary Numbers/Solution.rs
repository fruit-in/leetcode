impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.bytes().max().unwrap() as i32 - 48
    }
}
