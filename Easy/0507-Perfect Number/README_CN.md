# 507. 完美数
对于一个 **正整数**，如果它和除了它自身以外的所有正因子之和相等，我们称它为“完美数”。

给定一个 **整数** ```n```， 如果他是完美数，返回 ```True```，否则返回 ```False```

#### 示例:
<pre>
<strong>输入:</strong> 28
<strong>输出:</strong> True
<strong>解释:</strong> 28 = 1 + 2 + 4 + 7 + 14
</pre>

**提示:** 输入的数字 **```n```** 不会超过 100,000,000. (1e8)

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 1 {
            return false;
        }

        let mut sum = 1;
        let mut i = 2;

        while i * i < num {
            if num % i == 0 {
                sum += i + num / i;
            }
            i += 1;
        }

        if i * i == num {
            sum += i;
        }

        sum == num
    }
}
```
