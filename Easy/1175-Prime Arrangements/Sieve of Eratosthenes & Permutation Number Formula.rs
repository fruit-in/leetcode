impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut is_prime = vec![true; n as usize];
        is_prime[0] = false;
        let mut primes = n - 1;

        let mut i = 2;
        while i * i <= n as usize {
            if is_prime[i - 1] {
                let mut j = i * i;
                while j <= n as usize {
                    if is_prime[j - 1] {
                        is_prime[j - 1] = false;
                        primes -= 1;
                    }
                    j += i;
                }
            }
            i += 1;
        }

        let mut ret = 1_i64;
        for i in 1..=primes {
            ret = ret * i as i64 % 1000000007;
        }
        for i in 1..=(n - primes) {
            ret = ret * i as i64 % 1000000007;
        }

        ret as i32
    }
}
