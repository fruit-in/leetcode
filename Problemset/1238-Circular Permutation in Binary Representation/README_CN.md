# 1238. 循环码排列
给你两个整数 `n` 和 `start`。你的任务是返回任意 `(0,1,2,,...,2^n-1)` 的排列 `p`，并且满足：
* `p[0] = start`
* `p[i]` 和 `p[i+1]` 的二进制表示形式只有一位不同
* `p[0]` 和 `p[2^n -1]` 的二进制表示形式也只有一位不同

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2, start = 3
<strong>输出:</strong> [3,2,0,1]
<strong>解释:</strong> 这个排列的二进制表示是 (11,10,00,01)
     所有的相邻元素都有一位是不同的，另一个有效的排列是 [3,1,0,2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, start = 2
<strong>输出:</strong> [2,6,7,5,4,0,1,3]
<strong>解释:</strong> 这个排列的二进制表示是 (010,110,111,101,100,000,001,011)
</pre>

#### 提示:
* `1 <= n <= 16`
* `0 <= start < 2 ^ n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut x = 1;
        let mut start_index = 0;
        let mut ret = vec![0];

        for _ in 0..n {
            let rev = ret.iter().rev().map(|&num| num + x).collect::<Vec<i32>>();
            for i in 0..rev.len() {
                ret.push(rev[i]);
                if rev[i] == start {
                    start_index = ret.len() - 1;
                }
            }
            x *= 2;
        }

        ret.rotate_left(start_index);

        ret
    }
}
```
