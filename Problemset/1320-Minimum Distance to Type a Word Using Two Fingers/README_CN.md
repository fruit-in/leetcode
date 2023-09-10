# 1320. 二指输入的的最小距离
![](https://assets.leetcode.com/uploads/2020/01/02/leetcode_keyboard.png)

二指输入法定制键盘在 **X-Y** 平面上的布局如上图所示，其中每个大写英文字母都位于某个坐标处。

* 例如字母 **A** 位于坐标 **(0,0)**，字母 **B** 位于坐标 **(0,1)**，字母 **P** 位于坐标 **(2,3)** 且字母 **Z** 位于坐标 **(4,1)**。

给你一个待输入字符串 `word`，请你计算并返回在仅使用两根手指的情况下，键入该字符串需要的最小移动总距离。

坐标 <code>(x<sub>1</sub>,y<sub>1</sub>)</code> 和 <code>(x<sub>2</sub>,y<sub>2</sub>)</code> 之间的 **距离** 是 <code>|x<sub>1</sub> - x<sub>2</sub>| + |y<sub>1</sub> - y<sub>2</sub>|</code>。

**注意**，两根手指的起始位置是零代价的，不计入移动总距离。你的两根手指的起始位置也不必从首字母或者前两个字母开始。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "CAKE"
<strong>输出:</strong> 3
<strong>解释:</strong>
使用两根手指输入 "CAKE" 的最佳方案之一是：
手指 1 在字母 'C' 上 -> 移动距离 = 0
手指 1 在字母 'A' 上 -> 移动距离 = 从字母 'C' 到字母 'A' 的距离 = 2
手指 2 在字母 'K' 上 -> 移动距离 = 0
手指 2 在字母 'E' 上 -> 移动距离 = 从字母 'K' 到字母 'E' 的距离  = 1
总距离 = 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "HAPPY"
<strong>输出:</strong> 6
<strong>解释:</strong>
使用两根手指输入 "HAPPY" 的最佳方案之一是：
手指 1 在字母 'H' 上 -> 移动距离 = 0
手指 1 在字母 'A' 上 -> 移动距离 = 从字母 'H' 到字母 'A' 的距离 = 2
手指 2 在字母 'P' 上 -> 移动距离 = 0
手指 2 在字母 'P' 上 -> 移动距离 = 从字母 'P' 到字母 'P' 的距离 = 0
手指 1 在字母 'Y' 上 -> 移动距离 = 从字母 'A' 到字母 'Y' 的距离 = 4
总距离 = 6
</pre>

#### 提示:
* `2 <= word.length <= 300`
* 每个 `word[i]` 都是一个大写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word = word
            .bytes()
            .map(|b| (b - b'A') as usize)
            .collect::<Vec<_>>();
        let mut xy = (0..26).map(|i: i32| (i / 6, i % 6)).collect::<Vec<_>>();
        let mut dp = vec![vec![vec![i32::MAX; 26]; 26]; word.len() + 1];
        let mut ret = i32::MAX;
        dp[0] = vec![vec![0; 26]; 26];

        for i in 0..word.len() {
            let (x1, y1) = xy[word[i]];

            for j in 0..26 {
                let (x2, y2) = xy[j];

                for k in 0..26 {
                    if dp[i][j][k] == i32::MAX {
                        continue;
                    }

                    let (x3, y3) = xy[k];

                    dp[i + 1][word[i]][k] =
                        dp[i + 1][word[i]][k].min(dp[i][j][k] + (x1 - x2).abs() + (y1 - y2).abs());
                    dp[i + 1][j][word[i]] =
                        dp[i + 1][j][word[i]].min(dp[i][j][k] + (x1 - x3).abs() + (y1 - y3).abs());
                }
            }
        }

        for i in 0..26 {
            ret = ret.min(dp[word.len()][*word.last().unwrap()][i]);
            ret = ret.min(dp[word.len()][i][*word.last().unwrap()]);
        }

        ret
    }
}
```
