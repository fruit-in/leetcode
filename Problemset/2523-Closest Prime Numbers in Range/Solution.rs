impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut is_prime = vec![true; right as usize + 1];
        let mut primes = vec![];
        let mut ret = vec![-1, i32::MAX - 1];

        for nums2 in 2..=right {
            if ret[1] - ret[0] < 3 {
                break;
            }

            if is_prime[nums2 as usize] {
                if let Some(&nums1) = primes.last() {
                    if nums1 >= left && nums2 - nums1 < ret[1] - ret[0] {
                        ret = vec![nums1, nums2];
                    }
                }

                primes.push(nums2);
            }

            for prime in &primes {
                if prime * nums2 > right {
                    break;
                }

                is_prime[(prime * nums2) as usize] = false;

                if nums2 % prime == 0 {
                    break;
                }
            }
        }

        if ret[0] == -1 {
            ret[1] = -1;
        }

        ret
    }
}
