# 1175. 质数排列
请你帮忙给从 ```1``` 到 ```n``` 的数设计排列方案，使得所有的「质数」都应该被放在「质数索引」（索引从 1 开始）上；你需要返回可能的方案总数。

让我们一起来回顾一下「质数」：质数一定是大于 1 的，并且不能用两个小于它的正整数的乘积来表示。

由于答案可能会很大，所以请你返回答案 **模 mod ```10^9 + 7```** 之后的结果即可。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 12
<strong>解释:</strong> 举个例子，[1,2,5,4,3] 是一个有效的排列，但 [5,2,3,4,1] 不是，因为在第二种情况里质数 5 被错误地放在索引为 1 的位置上。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 100
<strong>输出:</strong> 682289015
</pre>

#### 提示:
* ```1 <= n <= 100```

## 题解 (Rust)

### 1. 厄拉多塞筛和排列数公式
```Rust
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut is_prime = vec![true; n as usize];
        is_prime[0] = false;
        let mut primes = n - 1;

        let mut i = 2;
        while i * i <= n as usize {
            if is_prime[i - 1] {
                let mut j = i * i;
                while j <= n as usize {
                    if is_prime[j - 1] {
                        is_prime[j - 1] = false;
                        primes -= 1;
                    }
                    j += i;
                }
            }
            i += 1;
        }

        let mut ret = 1_i64;
        for i in 1..=primes {
            ret = ret * i as i64 % 1000000007;
        }
        for i in 1..=(n - primes) {
            ret = ret * i as i64 % 1000000007;
        }

        ret as i32
    }
}
```
