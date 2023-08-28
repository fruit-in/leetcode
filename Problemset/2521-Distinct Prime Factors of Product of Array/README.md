# 2521. Distinct Prime Factors of Product of Array
Given an array of positive integers `nums`, return *the number of **distinct prime factors** in the product of the elements of* `nums`.

**Note** that:

* A number greater than `1` is called **prime** if it is divisible by only `1` and itself.
* An integer `val1` is a factor of another integer `val2` if `val2 / val1` is an integer.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,4,3,7,10,6]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The product of all the elements in nums is: 2 * 4 * 3 * 7 * 10 * 6 = 10080 = 2<sup>5</sup> * 3<sup>2</sup> * 5 * 7.
There are 4 distinct prime factors so we return 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,4,8,16]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The product of all the elements in nums is: 2 * 4 * 8 * 16 = 1024 = 2<sup>10</sup>.
There is 1 distinct prime factor so we return 1.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* `2 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
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
```
