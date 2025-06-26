# 1923. Longest Common Subpath
There is a country of `n` cities numbered from `0` to `n - 1`. In this country, there is a road connecting **every pair** of cities.

There are `m` friends numbered from `0` to `m - 1` who are traveling through the country. Each one of them will take a path consisting of some cities. Each path is represented by an integer array that contains the visited cities in order. The path may contain a city **more than once**, but the same city will not be listed consecutively.

Given an integer `n` and a 2D integer array `paths` where `paths[i]` is an integer array representing the path of the <code>i<sup>th</sup></code> friend, return *the length of the **longest common subpath** that is shared by **every** friend's path, or* `0` *if there is no common subpath at all*.

A **subpath** of a path is a contiguous sequence of cities within that path.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5, paths = [[0,1,2,3,4],
                       [2,3,4],
                       [4,0,1,2,3]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The longest common subpath is [2,3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, paths = [[0],[1],[2]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no common subpath shared by the three paths.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 5, paths = [[0,1,2,3,4],
                       [4,3,2,1,0]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The possible longest common subpaths are [0], [1], [2], [3], and [4]. All have a length of 1.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* `m == paths.length`
* <code>2 <= m <= 10<sup>5</sup></code>
* <code>sum(paths[i].length) <= 10<sup>5</sup></code>
* `0 <= paths[i][j] < n`
* The same city is not listed multiple times consecutively in `paths[i]`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        const BASE1: i64 = 100003;
        const MOD1: i64 = 1_000_000_007;
        const BASE2: i64 = 131071;
        const MOD2: i64 = 1_000_000_009;
        let max_len = paths.iter().map(|p| p.len()).max().unwrap();
        let min_len = paths.iter().map(|p| p.len()).min().unwrap();
        let mut base_pow1 = vec![1; max_len + 1];
        let mut base_pow2 = vec![1; max_len + 1];
        let mut l = 1;
        let mut r = min_len + 1;

        for i in 0..max_len {
            base_pow1[i + 1] = base_pow1[i] * BASE1 % MOD1;
            base_pow2[i + 1] = base_pow2[i] * BASE2 % MOD2;
        }

        while l < r {
            let m = (l + r) / 2;
            let mut count = HashMap::new();
            let mut flag = false;

            for i in 0..paths.len() {
                let mut hash1 = 0;
                let mut hash2 = 0;

                for j in 0..paths[i].len() {
                    hash1 = (hash1 * BASE1 + paths[i][j] as i64) % MOD1;
                    hash2 = (hash2 * BASE2 + paths[i][j] as i64) % MOD2;

                    if j >= m {
                        hash1 = (hash1 - paths[i][j - m] as i64 * base_pow1[m]).rem_euclid(MOD1);
                        hash2 = (hash2 - paths[i][j - m] as i64 * base_pow2[m]).rem_euclid(MOD2);
                    }
                    if j >= m - 1 {
                        if *count.get(&(hash1, hash2)).unwrap_or(&0) == i {
                            if i == paths.len() - 1 {
                                flag = true;
                                break;
                            }
                            count.insert((hash1, hash2), i + 1);
                        }
                    }
                }
            }

            if flag {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32 - 1
    }
}
```
