# 869. 重新排序得到 2 的幂
给定正整数 ```N``` ，我们按任何顺序（包括原始顺序）将数字重新排序，注意其前导数字不能为零。

如果我们可以通过上述方式得到 2 的幂，返回 ```true```；否则，返回 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 10
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 16
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> 24
<strong>输出:</strong> false
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> 46
<strong>输出:</strong> true
</pre>

#### 提示:
1. ```1 <= N <= 10^9```

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut power_of2 = Vec::new();

        for i in 0..30 {
            let mut arr = vec![0; 10];
            let mut x = 2_usize.pow(i);
            while x > 0 {
                arr[x % 10] += 1;
                x /= 10;
            }
            power_of2.push(arr);
        }

        let mut arr = vec![0; 10];
        let mut n = n as usize;
        while n > 0 {
            arr[n % 10] += 1;
            n /= 10;
        }

        power_of2.contains(&arr)
    }
}
```
