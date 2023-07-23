impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut cnt = 0;
        for i in 2..n {
            let mut j = 2;
            while j * j <= i && i % j != 0 {
                j += 1;
            }
            if j * j > i {
                cnt += 1;
            }
        }
        cnt
    }
}
