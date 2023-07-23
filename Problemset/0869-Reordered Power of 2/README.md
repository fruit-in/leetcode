# 869. Reordered Power of 2
Starting with a positive integer ```N```, we reorder the digits in any order (including the original order) such that the leading digit is not zero.

Return ```true``` if and only if we can do this in a way such that the resulting number is a power of 2.

#### Example 1:
<pre>
<strong>Input:</strong> 1
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 16
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> 24
<strong>Output:</strong> false
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> 46
<strong>Output:</strong> true
</pre>

#### Note:
1. ```1 <= N <= 10^9```

## Solutions (Rust)

### 1. Count
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
