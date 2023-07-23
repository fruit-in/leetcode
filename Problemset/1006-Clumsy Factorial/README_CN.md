# 1006. 笨阶乘
通常，正整数 ```n``` 的阶乘是所有小于或等于 ```n``` 的正整数的乘积。例如，```factorial(10) = 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1```。

相反，我们设计了一个笨阶乘 ```clumsy```：在整数的递减序列中，我们以一个固定顺序的操作符序列来依次替换原有的乘法操作符：乘法(*)，除法(/)，加法(+)和减法(-)。

例如，```clumsy(10) = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1```。然而，这些运算仍然使用通常的算术运算顺序：我们在任何加、减步骤之前执行所有的乘法和除法步骤，并且按从左到右处理乘法和除法步骤。

另外，我们使用的除法是地板除法（*floor division*），所以 ```10 * 9 / 8``` 等于 ```11```。这保证结果是一个整数。

实现上面定义的笨函数：给定一个整数 ```N```，它返回 ```N``` 的笨阶乘。

#### 示例 1:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> 7
<strong>解释:</strong> 7 = 4 * 3 / 2 + 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 10
<strong>输出:</strong> 12
<strong>解释:</strong> 12 = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1
</pre>

#### 提示:
1. ```1 <= N <= 10000```
2. ```-2^31 <= answer <= 2^31 - 1```  （答案保证符合 32 位整数。）

## 题解 (Ruby)

### 1. 数学
```Ruby
# @param {Integer} n
# @return {Integer}
def clumsy(n)
    if n % 4 == 0
        return [n + 1, 7].max
    elsif n < 3
        return n
    elsif n % 4 == 1 or n % 4 == 2
        return n + 2
    else
        return [n - 1, 6].max
    end
end
```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        match n % 4 {
            0 => (n + 1).max(7),
            1 | 2 if n < 4 => n,
            1 | 2 => n + 2,
            _ => (n - 1).max(6),
        }
    }
}
```
