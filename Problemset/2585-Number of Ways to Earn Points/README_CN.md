# 2585. 获得分数的方法数
考试中有 `n` 种类型的题目。给你一个整数 `target` 和一个下标从 **0** 开始的二维整数数组 `types` ，其中 <code>types[i] = [count<sub>i</sub>, marks<sub>i</sub>]</code> 表示第 `i` 种类型的题目有 <code>count<sub>i</sub></code> 道，每道题目对应 <code>marks<sub>i</sub></code> 分。

返回你在考试中恰好得到 `target` 分的方法数。由于答案可能很大，结果需要对 <code>10<sup>9</sup> +7</code> 取余。

**注意**，同类型题目无法区分。

* 比如说，如果有 `3` 道同类型题目，那么解答第 `1` 和第 `2` 道题目与解答第 `1` 和第 `3` 道题目或者第 `2` 和第 `3` 道题目是相同的。

#### 示例 1:
<pre>
<strong>输入:</strong> target = 6, types = [[6,1],[3,2],[2,3]]
<strong>输出:</strong> 7
<strong>解释:</strong> 要获得 6 分，你可以选择以下七种方法之一：
- 解决 6 道第 0 种类型的题目：1 + 1 + 1 + 1 + 1 + 1 = 6
- 解决 4 道第 0 种类型的题目和 1 道第 1 种类型的题目：1 + 1 + 1 + 1 + 2 = 6
- 解决 2 道第 0 种类型的题目和 2 道第 1 种类型的题目：1 + 1 + 2 + 2 = 6
- 解决 3 道第 0 种类型的题目和 1 道第 2 种类型的题目：1 + 1 + 1 + 3 = 6
- 解决 1 道第 0 种类型的题目、1 道第 1 种类型的题目和 1 道第 2 种类型的题目：1 + 2 + 3 = 6
- 解决 3 道第 1 种类型的题目：2 + 2 + 2 = 6
- 解决 2 道第 2 种类型的题目：3 + 3 = 6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = 5, types = [[50,1],[50,2],[50,5]]
<strong>输出:</strong> 4
<strong>解释:</strong> 要获得 5 分，你可以选择以下四种方法之一：
- 解决 5 道第 0 种类型的题目：1 + 1 + 1 + 1 + 1 = 5
- 解决 3 道第 0 种类型的题目和 1 道第 1 种类型的题目：1 + 1 + 1 + 2 = 5
- 解决 1 道第 0 种类型的题目和 2 道第 1 种类型的题目：1 + 2 + 2 = 5
- 解决 1 道第 2 种类型的题目：5
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> target = 18, types = [[6,1],[3,2],[2,3]]
<strong>输出:</strong> 1
<strong>解释:</strong> 只有回答所有题目才能获得 18 分。
</pre>

#### 提示:
* `1 <= target <= 1000`
* `n == types.length`
* `1 <= n <= 50`
* `types[i].length == 2`
* <code>1 <= count<sub>i</sub>, marks<sub>i</sub> <= 50</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for i in 0..types.len() {
            for j in (0..target).rev() {
                for k in 1..=types[i][0] {
                    let points = (k * types[i][1]) as usize + j;

                    if points > target {
                        break;
                    }

                    dp[points] = (dp[points] + dp[j]) % 1_000_000_007;
                }
            }
        }

        *dp.last().unwrap()
    }
}
```
