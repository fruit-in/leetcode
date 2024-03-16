# 646. 最长数对链
给你一个由 `n` 个数对组成的数对数组 `pairs` ，其中 <code>pairs[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> 且 <code>left<sub>i</sub> < right<sub>i</sub></code> 。

现在，我们定义一种 **跟随** 关系，当且仅当 `b < c` 时，数对 `p2 = [c, d]` 才可以跟在 `p1 = [a, b]` 后面。我们用这种形式来构造 **数对链** 。

找出并返回能够形成的 **最长数对链的长度** 。

你不需要用到所有的数对，你可以以任何顺序选择其中的一些数对来构造。

#### 示例 1:
<pre>
<strong>输入:</strong> pairs = [[1,2],[2,3],[3,4]]
<strong>输出:</strong> 2
<strong>解释:</strong> 最长的数对链是 [1,2] -> [3,4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pairs = [[1,2],[7,8],[4,5]]
<strong>输出:</strong> 3
<strong>解释:</strong> 最长的数对链是 [1,2] -> [4,5] -> [7,8] 。
</pre>

#### 提示:
* `n == pairs.length`
* `1 <= n <= 1000`
* <code>-1000 <= left<sub>i</sub> < right<sub>i</sub> <= 1000</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| (p[1], -p[0]));
        let mut right = pairs[0][1];
        let mut ret = 1;

        for i in 1..pairs.len() {
            if pairs[i][0] > right {
                right = pairs[i][1];
                ret += 1;
            }
        }

        ret
    }
}
```
