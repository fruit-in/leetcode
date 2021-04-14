# 688. “马”在棋盘上的概率
已知一个 `N`x`N` 的国际象棋棋盘，棋盘的行号和列号都是从 0 开始。即最左上角的格子记为 `(0, 0)`，最右下角的记为 `(N-1, N-1)`。

现有一个 “马”（也译作 “骑士”）位于 `(r, c)` ，并打算进行 `K` 次移动。

如下图所示，国际象棋的 “马” 每一步先沿水平或垂直方向移动 2 个格子，然后向与之相垂直的方向再移动 1 个格子，共有 8 个可选的位置。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/knight.png)

现在 “马” 每一步都从可选的位置（包括棋盘外部的）中独立随机地选择一个进行移动，直到移动了 `K` 次或跳到了棋盘外面。

求移动结束后，“马” 仍留在棋盘上的概率。

#### 示例:
<pre>
<strong>输入:</strong> 3, 2, 0, 0
<strong>输出:</strong> 0.0625
<strong>解释:</strong>
输入的数据依次为 N, K, r, c
第 1 步时，有且只有 2 种走法令 “马” 可以留在棋盘上（跳到（1,2）或（2,1））。对于以上的两种情况，各自在第2步均有且只有2种走法令 “马” 仍然留在棋盘上。
所以 “马” 在结束后仍在棋盘上的概率为 0.0625。
</pre>

#### 注意:
* `N` 的取值范围为 [1, 25]
* `K` 的取值范围为 [0, 100]
* 开始时，“马” 总是位于棋盘上

## 题解 (Ruby)

### 1. 动态规划
```Ruby
# @param {Integer} n
# @param {Integer} k
# @param {Integer} r
# @param {Integer} c
# @return {Float}
def knight_probability(n, k, r, c)
  dp = Array.new(n) { [0] * n }
  dp[r][c] = 1

  (1..k).each do |_|
    dp_ = Array.new(n) { [0] * n }
    (0...n).each do |r|
      (0...n).each do |c|
        [[-2, 1], [-1, 2], [1, 2], [2, 1], [2, -1], [1, -2], [-1, -2], [-2, -1]].each do |dr, dc|
          dp_[r][c] += probability(dp, n, r + dr, c + dc) / 8.0
        end
      end
    end
    dp = dp_
  end

  dp.flatten.sum
end

# @param {Integer[][]} dp
# @param {Integer} n
# @param {Integer} r
# @param {Integer} c
# @return {Float}
def probability(dp, n, r, c)
  r < 0 || c < 0 || r >= n || c >= n ? 0.0 : dp[r][c]
end
```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        let mut dp = vec![vec![0.0; n as usize]; n as usize];
        dp[r as usize][c as usize] = 1.0;

        for _ in 0..k {
            let mut dp_ = vec![vec![0.0; n as usize]; n as usize];
            for r in 0..n {
                for c in 0..n {
                    dp_[r as usize][c as usize] = (Self::probability(&dp, n, r - 2, c + 1)
                        + Self::probability(&dp, n, r - 1, c + 2)
                        + Self::probability(&dp, n, r + 1, c + 2)
                        + Self::probability(&dp, n, r + 2, c + 1)
                        + Self::probability(&dp, n, r + 2, c - 1)
                        + Self::probability(&dp, n, r + 1, c - 2)
                        + Self::probability(&dp, n, r - 1, c - 2)
                        + Self::probability(&dp, n, r - 2, c - 1))
                        / 8.0;
                }
            }
            dp = dp_;
        }

        dp.concat().iter().sum::<f64>()
    }

    fn probability(dp: &[Vec<f64>], n: i32, r: i32, c: i32) -> f64 {
        if r < 0 || c < 0 || r >= n || c >= n {
            0.0
        } else {
            dp[r as usize][c as usize]
        }
    }
}
```
