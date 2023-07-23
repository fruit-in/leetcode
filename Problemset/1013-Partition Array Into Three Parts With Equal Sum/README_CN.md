# 1013. 将数组分成和相等的三个部分
给定一个整数数组 ```A```，只有我们可以将其划分为三个和相等的非空部分时才返回 ```true```，否则返回 ```false```。

形式上，如果我们可以找出索引 ```i+1 < j``` 且满足 ```(A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... + A[j-1] == A[j] + A[j-1] + ... + A[A.length - 1])``` 就可以将数组三等分。

#### 示例 1:
<pre>
<strong>输入:</strong> [0,2,1,-6,6,-7,9,1,2,0,1]
<strong>输出:</strong> true
<strong>解释:</strong> 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [0,2,1,-6,6,7,9,-1,2,0,1]
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [3,3,6,5,-2,2,5,1,-9,4]
<strong>输出:</strong> true
<strong>解释:</strong> 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
</pre>

#### 提示:
1. ```3 <= A.length <= 50000```
2. ```-10000 <= A[i] <= 10000```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let mut total_sum: i32 = a.iter().sum();
        if total_sum % 3 != 0 {
            return false;
        }

        let mut i = 0;
        let mut part_sum = 0;
        while i < a.len() {
            part_sum += a[i];
            if part_sum == total_sum / 3 {
                break;
            }
            i += 1;
        }

        let mut j = a.len() - 1;
        part_sum = 0;
        while j > 0 {
            part_sum += a[j];
            if part_sum == total_sum / 3 {
                break;
            }
            j -= 1;
        }

        i + 1 < j
    }
}
```
