# 935. 骑士拨号器
国际象棋中的骑士可以按下图所示进行移动：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/11/03/knight.png) ![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/11/03/keypad.png)

这一次，我们将 “骑士” 放在电话拨号盘的任意数字键（如上图所示）上，接下来，骑士将会跳 N-1 步。每一步必须是从一个数字键跳到另一个数字键。

每当它落在一个键上（包括骑士的初始位置），都会拨出键所对应的数字，总共按下 `N` 位数字。

你能用这种方式拨出多少个不同的号码？

因为答案可能很大，**所以输出答案模 10^9 + 7**。

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> 10
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> 20
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> 46
</pre>

#### 提示:
* `1 <= N <= 5000`

## 题解 (Ruby)

### 1. 动态规划
```Ruby
# @param {Integer} n
# @return {Integer}
def knight_dialer(n)
  dp = [1] * 10

  (1...n).each do |_|
    dp = [
      dp[4] + dp[6],
      dp[6] + dp[8],
      dp[7] + dp[9],
      dp[4] + dp[8],
      dp[0] + dp[3] + dp[9],
      0,
      dp[0] + dp[1] + dp[7],
      dp[2] + dp[6],
      dp[1] + dp[3],
      dp[2] + dp[4]
    ]
  end

  dp.sum % 1_000_000_007
end
```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut dp = [1; 10];

        for _ in 1..n {
            let mut dp_ = [0; 10];
            for i in 0..10 {
                dp_[i] = match i {
                    0 => dp[4] + dp[6],
                    1 | 3 => dp[7 - i] + dp[8],
                    2 | 8 => dp[9 - i] + dp[11 - i],
                    4 | 6 => (dp[0] + dp[7 - i]) % 1_000_000_007 + dp[13 - i],
                    7 | 9 => dp[2] + dp[13 - i],
                    _ => 0,
                } % 1_000_000_007;
            }
            dp = dp_;
        }

        dp.iter().fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
```
