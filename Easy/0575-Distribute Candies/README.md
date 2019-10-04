# 575. Distribute Candies
Given an integer array with **even** length, where different numbers in this array represent different **kinds** of candies. Each number means one candy of the corresponding kind. You need to distribute these candies **equally** in number to brother and sister. Return the maximum number of **kinds** of candies the sister could gain.

#### Example 1:
<pre>
<strong>Input:</strong> candies = [1,1,2,2,3,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
There are three different kinds of candies (1, 2 and 3), and two candies for each kind.
Optimal distribution: The sister has candies [1,2,3] and the brother has candies [1,2,3], too.
The sister has three different kinds of candies.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> candies = [1,1,2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
For example, the sister has candies [2,3] and the brother has candies [1,1].
The sister has two different kinds of candies, the brother has only one kind of candies.
</pre>

#### Note:
1. The length of the given array is in range [2, 10,000], and will be even.
2. The number in given array is in range [-100,000, 100,000].

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let sister: HashSet<_> = candies.iter().collect();
        sister.len().min(candies.len() / 2) as i32
    }
}
```

### 2. Sort
```Rust
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let mut candies = candies;
        candies.sort_unstable();

        let mut sister = 1;

        for i in 1..candies.len() {
            if candies[i] != candies[i - 1] {
                sister += 1;
            }
        }

        sister.min(candies.len() / 2) as i32
    }
}
```
