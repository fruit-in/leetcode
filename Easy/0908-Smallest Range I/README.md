# 908. Smallest Range I
Given an array <code>A</code> of integers, for each integer <code>A[i]</code> we may choose any <code>x</code> with <code>-K <= x <= K</code>, and add <code>x</code> to <code>A[i]</code>.

After this process, we have some array <code>B</code>.

Return the smallest possible difference between the maximum value of <code>B</code> and the minimum value of <code>B</code>.

#### Example 1:
<pre>
<strong>Input:</strong> A = [1], K = 0
<strong>Output:</strong> 0
<strong>Explanation:</strong> B = [1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = [0,10], K = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> B = [2,8]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> A = [1,3,6], K = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> B = [3,3,3] or B = [4,4,4]
</pre>

#### Note:
1. <code>1 <= A.length <= 10000</code>
2. <code>0 <= A[i] <= 10000</code>
3. <code>0 <= K <= 10000</code>

## Solutions

### 1. Mathematical (Rust)
```Rust
impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let mut min = a.iter().min().unwrap();
        let mut max = a.iter().max().unwrap();
        if max - min < 2 * k {
            0
        } else {
            max - min - 2 * k
        }
    }
}
```
