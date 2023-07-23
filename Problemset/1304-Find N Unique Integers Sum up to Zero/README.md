# 1304. Find N Unique Integers Sum up to Zero
Given an integer ```n```, return **any** array containing ```n``` **unique** integers such that they add up to 0.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> [-7,-1,1,3,4]
<strong>Explanation:</strong> These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [-1,0,1]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> [0]
</pre>

#### Constraints:
* ```1 <= n <= 1000```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        ((1 - n)..=n).step_by(2).collect()
    }
}
```
