# 1806. 还原排列的最少操作步数
给你一个偶数 `n` ，已知存在一个长度为 `n` 的排列 `perm` ，其中 `perm[i] == i` （下标 **从 0 开始** 计数）。

一步操作中，你将创建一个新数组 `arr` ，对于每个 `i` ：

* 如果 `i % 2 == 0` ，那么 `arr[i] = perm[i / 2]`
* 如果 `i % 2 == 1` ，那么 `arr[i] = perm[n / 2 + (i - 1) / 2]`

然后将 `arr` 赋值给 `perm` 。

要想使 `perm` 回到排列初始值，至少需要执行多少步操作？返回最小的 **非零** 操作步数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 最初，perm = [0,1]
第 1 步操作后，perm = [0,1]
所以，仅需执行 1 步操作
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 2
<strong>解释:</strong> 最初，perm = [0,1,2,3]
第 1 步操作后，perm = [0,2,1,3]
第 2 步操作后，perm = [0,1,2,3]
所以，仅需执行 2 步操作
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 6
<strong>输出:</strong> 4
</pre>

#### 提示:
* `2 <= n <= 1000`
* `n` 是一个偶数

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let n = n as usize;
        let mut perm = (0..n).collect::<Vec<_>>();

        for ret in 1..=n as i32 {
            let mut arr = vec![0; n];

            for i in 0..n {
                if i % 2 == 0 {
                    arr[i] = perm[i / 2];
                } else {
                    arr[i] = perm[n / 2 + (i - 1) / 2];
                }
            }

            perm = arr;

            if perm.iter().enumerate().all(|(i, &x)| i == x) {
                return ret;
            }
        }

        unreachable!()
    }
}
```
