impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut counter = 0;
        let mut z = x ^ y;
        while z != 0 {
            counter += z & 1;
            z >>= 1;
        }
        counter
    }
}
