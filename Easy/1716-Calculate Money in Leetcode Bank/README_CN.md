# 1716. 计算力扣银行的钱
Hercy 想要为购买第一辆车存钱。他 **每天** 都往力扣银行里存钱。

最开始，他在周一的时候存入 `1` 块钱。从周二到周日，他每天都比前一天多存入 `1` 块钱。在接下来每一个周一，他都会比 **前一个周一** 多存入 1 块钱。

给你 `n` ，请你返回在第 `n` 天结束的时候他在力扣银行总共存了多少块钱。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 10
<strong>解释:</strong> 第 4 天后，总额为 1 + 2 + 3 + 4 = 10 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 10
<strong>输出:</strong> 37
<strong>解释:</strong> 第 10 天后，总额为 (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37 。注意到第二个星期一，Hercy 存入 2 块钱。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 20
<strong>输出:</strong> 96
<strong>解释:</strong> 第 20 天后，总额为 (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96 。
</pre>

#### 提示:
* `1 <= n <= 1000`

## 题解 (Ruby)

### 1. 数学
```Ruby
# @param {Integer} n
# @return {Integer}
def total_money(n)
  x = n / 7
  y = n % 7

  28 * x + x * (x - 1) * 7 / 2 + (x + 1) * y + y * (y - 1) / 2
end
```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let x = n / 7;
        let y = n % 7;

        28 * x + x * (x - 1) * 7 / 2 + (x + 1) * y + y * (y - 1) / 2
    }
}
```
