impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut primes = vec![];

        for i in 2..=max_num {
            if (2..=(i as f64).sqrt() as i32).all(|j| i % j != 0) {
                primes.push(i);
            }
        }

        for i in 0..nums.len() {
            for j in 0..primes.len() {
                if nums[i] < primes[j] {
                    break;
                }

                if nums[i] % primes[j] == 0 {
                    primes[j] = 1;
                }
            }
        }

        primes.iter().filter(|&&x| x == 1).count() as i32
    }
}
