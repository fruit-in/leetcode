# 2611. 老鼠和奶酪
有两只老鼠和 `n` 块不同类型的奶酪，每块奶酪都只能被其中一只老鼠吃掉。

下标为 `i` 处的奶酪被吃掉的得分为：

* 如果第一只老鼠吃掉，则得分为 `reward1[i]` 。
* 如果第二只老鼠吃掉，则得分为 `reward2[i]` 。

给你一个正整数数组 `reward1` ，一个正整数数组 `reward2` ，和一个非负整数 `k` 。

请你返回第一只老鼠恰好吃掉 `k` 块奶酪的情况下，**最大** 得分为多少。

#### 示例 1:
<pre>
<strong>输入:</strong> reward1 = [1,1,3,4], reward2 = [4,4,1,1], k = 2
<strong>输出:</strong> 15
<strong>解释:</strong> 这个例子中，第一只老鼠吃掉第 2 和 3 块奶酪（下标从 0 开始），第二只老鼠吃掉第 0 和 1 块奶酪。
总得分为 4 + 4 + 3 + 4 = 15 。
15 是最高得分。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> reward1 = [1,1], reward2 = [1,1], k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 这个例子中，第一只老鼠吃掉第 0 和 1 块奶酪（下标从 0 开始），第二只老鼠不吃任何奶酪。
总得分为 1 + 1 = 2 。
2 是最高得分。
</pre>

#### 提示:
* <code>1 <= n == reward1.length == reward2.length <= 10<sup>5</sup></code>
* `1 <= reward1[i], reward2[i] <= 1000`
* `0 <= k <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut indices = (0..reward1.len()).collect::<Vec<_>>();
        let mut ret = 0;

        indices.sort_unstable_by_key(|&i| reward2[i] - reward1[i]);

        for i in 0..reward1.len() {
            if i < k as usize {
                ret += reward1[indices[i]];
            } else {
                ret += reward2[indices[i]];
            }
        }

        ret
    }
}
```
