# 1923. 最长公共子路径
一个国家由 `n` 个编号为 `0` 到 `n - 1` 的城市组成。在这个国家里，**每两个** 城市之间都有一条道路连接。

总共有 `m` 个编号为 `0` 到 `m - 1` 的朋友想在这个国家旅游。他们每一个人的路径都会包含一些城市。每条路径都由一个整数数组表示，每个整数数组表示一个朋友按顺序访问过的城市序列。同一个城市在一条路径中可能 **重复** 出现，但同一个城市在一条路径中不会连续出现。

给你一个整数 `n` 和二维数组 `paths` ，其中 `paths[i]` 是一个整数数组，表示第 `i` 个朋友走过的路径，请你返回 **每一个** 朋友都走过的 **最长公共子路径** 的长度，如果不存在公共子路径，请你返回 `0` 。

一个 **子路径** 指的是一条路径中连续的城市序列。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5, paths = [[0,1,2,3,4],
                     [2,3,4],
                     [4,0,1,2,3]]
<strong>输出:</strong> 2
<strong>解释:</strong> 最长公共子路径为 [2,3] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, paths = [[0],[1],[2]]
<strong>输出:</strong> 0
<strong>解释:</strong> 三条路径没有公共子路径。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5, paths = [[0,1,2,3,4],
                     [4,3,2,1,0]]
<strong>输出:</strong> 1
<strong>解释:</strong> 最长公共子路径为 [0]，[1]，[2]，[3] 和 [4] 。它们长度都为 1 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* `m == paths.length`
* <code>2 <= m <= 10<sup>5</sup></code>
* <code>sum(paths[i].length) <= 10<sup>5</sup></code>
* `0 <= paths[i][j] < n`
* `paths[i]` 中同一个城市不会连续重复出现。

## 题解 (Rust)

### 1. 题解
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
