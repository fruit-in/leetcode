# 354. 俄罗斯套娃信封问题
给你一个二维整数数组 `envelopes` ，其中 <code>envelopes[i] = [w<sub>i</sub>, h<sub>i</sub>]</code> ，表示第 `i` 个信封的宽度和高度。

当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。

请计算 **最多能有多少个** 信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。

**注意：**不允许旋转信封。

#### 示例 1:
<pre>
<strong>输入:</strong> envelopes = [[5,4],[6,4],[6,7],[2,3]]
<strong>输出:</strong> 3
<strong>解释:</strong> 最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> envelopes = [[1,1],[1,1],[1,1]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= envelopes.length <= 10<sup>5</sup></code>
* `envelopes[i].length == 2`
* <code>1 <= w<sub>i</sub>, h<sub>i</sub> <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let mut lis = vec![];

        envelopes.sort_unstable_by_key(|e| (e[0], -e[1]));

        for i in 0..envelopes.len() {
            if let Err(j) = lis.binary_search(&envelopes[i][1]) {
                if j == lis.len() {
                    lis.push(envelopes[i][1]);
                } else {
                    lis[j] = envelopes[i][1];
                }
            }
        }

        lis.len() as i32
    }
}
```
