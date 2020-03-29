# 1017. 负二进制转换
给出数字 ```N```，返回由若干 ```"0"``` 和 ```"1"```组成的字符串，该字符串为 ```N``` 的<strong>负二进制（```base -2```）</strong>表示。

除非字符串就是 ```"0"```，否则返回的字符串中不能含有前导零。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> "110"
<strong>解释:</strong> (-2) ^ 2 + (-2) ^ 1 = 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> "111"
<strong>解释:</strong> (-2) ^ 2 + (-2) ^ 1 + (-2) ^ 0 = 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> "100"
<strong>解释:</strong> (-2) ^ 2 = 4
</pre>

#### 提示:
1. ```0 <= N <= 10^9```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let mut n = n;
        let mut ret = Vec::new();

        while n != 0 {
            let x = n % 4;
            match x {
                0 => ret.push("00"),
                1 if n != 1 => ret.push("01"),
                2 => ret.push("10"),
                3 => ret.push("11"),
                _ => ret.push("1"),
            }
            n = n / 4 + x / 2;
        }

        ret.reverse();
        ret.concat()
    }
}
```
