# 1223. 掷骰子模拟
有一个骰子模拟器会每次投掷的时候生成一个 1 到 6 的随机数。

不过我们在使用它时有个约束，就是使得投掷骰子时，**连续** 掷出数字 ```i``` 的次数不能超过 ```rollMax[i]```（```i``` 从 1 开始编号）。

现在，给你一个整数数组 ```rollMax``` 和一个整数 ```n```，请你来计算掷 ```n``` 次骰子可得到的不同点数序列的数量。

假如两个序列中至少存在一个元素不同，就认为这两个序列是不同的。由于答案可能很大，所以请返回 **模 ```10^9 + 7```** 之后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2, rollMax = [1,1,2,2,2,3]
<strong>输出:</strong> 34
<strong>解释:</strong> 我们掷 2 次骰子，如果没有约束的话，共有 6 * 6 = 36 种可能的组合。但是根据 rollMax 数组，数字 1 和 2 最多连续出现一次，所以不会出现序列 (1,1) 和 (2,2)。因此，最终答案是 36-2 = 34。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, rollMax = [1,1,1,1,1,1]
<strong>输出:</strong> 30
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3, rollMax = [1,1,1,2,2,3]
<strong>输出:</strong> 181
</pre>

#### 提示:
* ```1 <= n <= 5000```
* ```rollMax.length == 6```
* ```1 <= rollMax[i] <= 15```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; 16]; 7]; n as usize];
        (1..=6).for_each(|j| dp[0][j][0] = 1);
        let mut ret = 0;

        for i in 1..(n as usize) {
            for j in 1..=6 {
                let f = |x: usize| {
                    dp[i - 1][x][..(roll_max[x - 1] as usize)]
                        .iter()
                        .fold(0, |acc, x| (acc + x) % M)
                };
                dp[i][j][0] = (1..=6)
                    .filter(|&x| x != j)
                    .map(|x| f(x))
                    .fold(0, |acc, x| (acc + x) % M);
                for k in 1..(roll_max[j - 1] as usize).min(i + 1) {
                    dp[i][j][k] = dp[i - 1][j][k - 1];
                }
            }
        }

        for j in 1..=6 {
            for k in 0..(roll_max[j - 1].min(n) as usize) {
                ret = (ret + dp[n as usize - 1][j][k]) % M;
            }
        }

        ret
    }
}
```
